use async_trait::async_trait;
use debug_types::{
    requests::InitializeRequestArguments,
    responses::{InitializeResponse, Response, ResponseBody},
    types::Capabilities,
};
use either::Either;

use crate::debugger::{Client, DebugAdapter, State};

#[async_trait]
impl DebugAdapter for NixDebugAdapter {
    async fn handle_request(&mut self, seq: i64, command: debug_types::requests::RequestCommand) {
        use debug_types::requests::RequestCommand::*;
        match command {
            Initialize(initialize_args) => self.handle_initialize(seq, initialize_args).await,
            _ => {
                self.client
                    .send(Either::Right(Response {
                        request_seq: seq,
                        success: false,
                        message: Some("unsupported request".to_string()),
                        body: None,
                    }))
                    .await;
            }
        }
    }
}

impl NixDebugAdapter {
    async fn handle_initialize(&mut self, seq: i64, _args: InitializeRequestArguments) {
        let capabilities = Capabilities {
            supports_configuration_done_request: Some(true),
            supports_step_in_targets_request: Some(true),
            support_terminate_debuggee: Some(true),
            supports_loaded_sources_request: Some(true),
            supports_data_breakpoints: Some(true),
            supports_breakpoint_locations_request: Some(true),
            ..default_capabilities()
        };

        let response = InitializeResponse { capabilities };

        let body = Some(ResponseBody::Initialize(response));

        self.client
            .send(Either::Right(Response {
                request_seq: seq,
                success: true,
                message: None,
                body,
            }))
            .await;

        self.client.set_state(State::Initialized);
    }
}

pub struct NixDebugAdapter {
    pub client: Client,
    pub state: NixDebugState,
}

#[derive(Default, Debug, Clone)]
pub struct NixDebugState {}

// FIXME why does capabilities not implement default?
pub fn default_capabilities() -> Capabilities {
    Capabilities {
        supports_configuration_done_request: None,
        supports_function_breakpoints: None,
        supports_step_in_targets_request: None,
        support_terminate_debuggee: None,
        supports_loaded_sources_request: None,
        supports_data_breakpoints: None,
        supports_breakpoint_locations_request: None,
        supports_conditional_breakpoints: None,
        supports_hit_conditional_breakpoints: None,
        supports_evaluate_for_hovers: None,
        exception_breakpoint_filters: None,
        supports_step_back: None,
        supports_set_variable: None,
        supports_restart_frame: None,
        supports_goto_targets_request: None,
        supports_completions_request: None,
        completion_trigger_characters: None,
        supports_modules_request: None,
        additional_module_columns: None,
        supported_checksum_algorithms: None,
        supports_restart_request: None,
        supports_exception_options: None,
        supports_value_formatting_options: None,
        supports_exception_info_request: None,
        support_suspend_debuggee: None,
        supports_delayed_stack_trace_loading: None,
        supports_log_points: None,
        supports_terminate_threads_request: None,
        supports_set_expression: None,
        supports_terminate_request: None,
        supports_read_memory_request: None,
        supports_write_memory_request: None,
        supports_disassemble_request: None,
        supports_cancel_request: None,
        supports_clipboard_context: None,
        supports_stepping_granularity: None,
        supports_instruction_breakpoints: None,
        supports_exception_filter_options: None,
        supports_single_thread_execution_requests: None,
    }
}
