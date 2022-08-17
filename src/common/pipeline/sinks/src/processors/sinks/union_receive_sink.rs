// Copyright 2022 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;

use common_base::base::tokio::sync::mpsc::Sender;
use common_datablocks::DataBlock;
use common_exception::ErrorCode;
use common_exception::Result;
use common_pipeline_core::processors::port::InputPort;
use common_pipeline_core::processors::processor::ProcessorPtr;

use crate::processors::sinks::Sink;
use crate::processors::sinks::Sinker;

pub struct UnionReceiveSink {
    input_blocks: Vec<DataBlock>,
    sender: Sender<Option<DataBlock>>,
}

impl UnionReceiveSink {
    pub fn create(sender: Sender<Option<DataBlock>>, input: Arc<InputPort>) -> ProcessorPtr {
        Sinker::create(input, UnionReceiveSink {
            input_blocks: vec![],
            sender,
        })
    }
}

#[async_trait::async_trait]
impl Sink for UnionReceiveSink {
    const NAME: &'static str = "UnionReceiveSink";

    fn on_finish(&mut self) -> Result<()> {
        let send_blocks = if self.input_blocks.is_empty() {
            None
        } else {
            Some(DataBlock::concat_blocks(&self.input_blocks)?)
        };
        if let Err(_) = self.sender.try_send(send_blocks) {
            return Err(ErrorCode::UnexpectedError("UnionReceiveSink sender failed"));
        };

        Ok(())
    }

    fn consume(&mut self, data_block: DataBlock) -> Result<()> {
        self.input_blocks.push(data_block);
        Ok(())
    }
}
