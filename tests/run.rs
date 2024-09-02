use dotenv::dotenv;
use env_logger::Env;
use ollama_workflows::{Entry, Executor, Model, ProgramMemory, Workflow};

// Constants for workflow paths
const SEARCH_WORKFLOW_PATH: &str = "./tests/test_workflows/search.json";
const ALL_TOOLS_WORKFLOW_PATH: &str = "./tests/test_workflows/all.json";
const QUESTIONS_WORKFLOW_PATH: &str = "./tests/test_workflows/questions.json";
const POST_PROCESS_WORKFLOW_PATH: &str = "./tests/test_workflows/post_process.json";
const TICKER_WORKFLOW_PATH: &str = "./tests/test_workflows/ticker.json";
const SIMPLE_WORKFLOW_PATH: &str = "./tests/test_workflows/simple.json";
const INSERT_WORKFLOW_PATH: &str = "./tests/test_workflows/insert.json";
const USERS_WORKFLOW_PATH: &str = "./tests/test_workflows/users.json";

async fn setup_test(model: Model) -> Executor {
    dotenv().ok();
    let env = Env::default().filter_or("LOG_LEVEL", "info");
    env_logger::Builder::from_env(env).init();
    Executor::new(model)
}

macro_rules! workflow_test {
    ($name:ident, $model:expr, $workflow:expr, $input:expr) => {
        #[tokio::test]
        async fn $name() {
            let exe = setup_test($model).await;
            let workflow = Workflow::new_from_json($workflow).unwrap();
            let mut memory = ProgramMemory::new();
            let input = Entry::try_value_or_str($input);
            if let Err(e) = exe.execute(Some(&input), workflow, &mut memory).await {
                log::error!("Execution failed: {}", e);
            };
        }
    };
    ($name:ident, $model:expr, $workflow:expr) => {
        #[tokio::test]
        async fn $name() {
            let exe = setup_test($model).await;
            let workflow = Workflow::new_from_json($workflow).unwrap();
            let mut memory = ProgramMemory::new();
            if let Err(e) = exe.execute(None, workflow, &mut memory).await {
                log::error!("Execution failed: {}", e);
            };
        }
    };
}

// Search workflow tests
mod ticker_tests {
    use super::*;

    workflow_test!(
        test_ticker_workflow_openai,
        Model::GPT4oMini,
        TICKER_WORKFLOW_PATH
    );
}

mod simple_workflow_tests {
    use super::*;

    workflow_test!(
        test_simple_workflow,
        Model::Phi3Medium,
        SIMPLE_WORKFLOW_PATH,
        "How does reiki work?"
    );
}

mod insert_workflow_tests {
    use super::*;

    workflow_test!(
        test_insert_workflow_ollama,
        Model::Phi3Medium,
        INSERT_WORKFLOW_PATH
    );
    workflow_test!(
        test_insert_workflow_openai,
        Model::GPT4oMini,
        INSERT_WORKFLOW_PATH
    );
}

mod user_workflow_tests {
    use super::*;

    workflow_test!(test_user_workflow, Model::GPT4o, USERS_WORKFLOW_PATH);
}

mod function_call_tests {
    use super::*;

    workflow_test!(
        test_function_call_phi3_5_fp16,
        Model::Phi3_5MiniFp16,
        SEARCH_WORKFLOW_PATH,
        "How does reiki work?"
    );
    workflow_test!(
        function_calling_nous_theta,
        Model::NousTheta,
        SEARCH_WORKFLOW_PATH,
        "How many Hoodoo's are in Kapadokya?"
    );
    workflow_test!(
        function_calling_llama3_1,
        Model::Llama3_1_8B,
        SEARCH_WORKFLOW_PATH,
        "How many fairy chimneys are there in Cappadocia?"
    );
}

mod all_tools_workflow_tests {
    use super::*;

    workflow_test!(
        test_all_tools_workflow,
        Model::GPT4oMini,
        ALL_TOOLS_WORKFLOW_PATH,
        "What's the weather like in New York and how does it affect the stock market?"
    );
}

mod questions_workflow_tests {
    use super::*;

    workflow_test!(
        test_questions_workflow,
        Model::Phi3Medium,
        QUESTIONS_WORKFLOW_PATH,
        "Tell me about the history of artificial intelligence."
    );
}

mod post_process_workflow_tests {
    use super::*;

    workflow_test!(
        test_post_process_workflow,
        Model::Llama3_1_8B,
        POST_PROCESS_WORKFLOW_PATH,
        "Summarize the main plot points of Romeo and Juliet."
    );
}
