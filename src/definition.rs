// Copyright (c) The StackClass Authors. All rights reserved.
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

use tester::{Case, Definition};

use crate::stages::{base::*, evaluating_expressions::*, parsing_expressions::*};

pub fn build() -> Definition {
    Definition {
        executable_name: "your_program.sh".to_string(),
        legacy_executable_name: Some("your_program.sh".to_string()),
        cases: vec![
            // Base Stages
            Case::new("ry8", Arc::new(ry8::test_eof)),
            Case::new("ol4", Arc::new(ol4::test_paren)),
            Case::new("oe8", Arc::new(oe8::test_brace)),
            // Parsing Expressions
            Case::new("sc2", Arc::new(sc2::test_parse_booleans)),
            Case::new("ra8", Arc::new(ra8::test_parse_numbers)),
            Case::new("th5", Arc::new(th5::test_parse_strings)),
            // Evaluating Expressions
            Case::new("iz6", Arc::new(iz6::test_evaluate_booleans)),
            Case::new("lv1", Arc::new(lv1::test_evaluate_literals)),
            Case::new("oq9", Arc::new(oq9::test_evaluate_parens)),
        ],
        ..Default::default()
    }
}
