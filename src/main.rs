mod flask;
mod helpers;


use flask::flask_init;
use helpers::collect_user_arguments;
use helpers::check_if_arguments_count_valid;


fn main() {
    let args: Vec<String> = collect_user_arguments();

    if check_if_arguments_count_valid(&args) {
        let project_name = &args[1];
        flask_init(project_name.to_string());
    
    }

}