use std::io;
mod features;
use features::task::controller as tc;

fn get_user_input() -> Result<String, String> {
    let mut option = String::new();

    let result = match io::stdin().read_line(&mut option) {
        Ok(_) => Ok(option),
        Err(err) => Err(format!("Erro ao receber o input, tente novamente!\nErro: {}", err)),
    };
    clearscreen::clear().expect("Falha ao limpar tela");
    result
}

fn pause() {
    let mut _unused = String::new();
    io::stdin().read_line(&mut _unused).unwrap();
}

fn main() {
    loop {
        clearscreen::clear().expect("Falha ao limpar tela");
        tc::greet();
        println!(
            "\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
            "1. Visualizar Tarefas",
            "2. Buscar Tarefa",
            "3. Adicionar Tarefa",
            "4. Editar Tarefa",
            "5. Remove Tarefa",
            "6. Marcar Tarefa como A Fazer",
            "7. Marcar Tarefa como Fazendo",
            "8. Marcar Tarefa como Feito",
            "9. Mudar Username",
            "10. Resetar Para Configuração de Fábrica",
            "0. Sair"
        );
        
        let option = match get_user_input() {
            Ok(option) => option,
            Err(err) => { println!("{}", err); continue }
        };

        match option.trim() {
            "0" => {
                println!("Até Mais!");
                break
            },
            "1" => {
                tc::print_tasks();
                pause();
            },
            "2" => {
                println!("Qual o id da tarefa que deseja buscar?");
                let id = match get_user_input() {
                    Ok(id) => id,
                    Err(err) => { println!("{}", err); continue }
                };
                tc::print_task_by_id(id);
                pause();
            },
            "3" => {
                println!("Qual a descrição da tarefa que deseja adicionar?");
                let description = match get_user_input() {
                    Ok(description) => description,
                    Err(err) => { println!("{}", err); continue }
                };
                tc::create_task(description);
                pause();
            },
            "4" => {
                println!("Qual o id da tarefa que deseja editar?");
                let id = match get_user_input() {
                    Ok(id) => id,
                    Err(err) => { println!("{}", err); continue }
                };

                println!("Qual a nova descrição da tarefa?");
                let description = match get_user_input() {
                    Ok(description) => description,
                    Err(err) => { println!("{}", err); continue }
                };
                tc::edit_task(id, description);
                pause();
            },
            "5" => {
                println!("Qual o id da tarefa que deseja deletar?");
                let id = match get_user_input() {
                    Ok(id) => id,
                    Err(err) => { println!("{}", err); continue }
                };

                println!("Você tem certeza? (sim\\nao)");
                let confirm = match get_user_input() {
                    Ok(confirm) => confirm,
                    Err(err) => { println!("{}", err); continue }
                };

                if confirm.trim() == "sim" {
                    tc::delete_task(id);
                    pause();
                }
            },
            "6" => {
                println!("Qual o id da tarefa que deseja marcar como A Fazer?");
                let id = match get_user_input() {
                    Ok(id) => id,
                    Err(err) => { println!("{}", err); continue }
                };
                
                tc::toggle_to_do(id);
                pause();
            },
            "7" => {
                println!("Qual o id da tarefa que deseja marcar como Fazendo?");
                let id = match get_user_input() {
                    Ok(id) => id,
                    Err(err) => { println!("{}", err); continue }
                };
                
                tc::toggle_doing(id);
                pause();
            },
            "8" => {
                println!("Qual o id da tarefa que deseja marcar como Feito?");
                let id = match get_user_input() {
                    Ok(id) => id,
                    Err(err) => { println!("{}", err); continue }
                };
                
                tc::toggle_done(id);
                pause();
            },
            "9" => {
                println!("Qual o novo nome de usuário?");
                let new_username = match get_user_input() {
                    Ok(new_username) => new_username,
                    Err(err) => { println!("{}", err); continue }
                };
                
                tc::change_username(new_username);
                pause();
            },
            "10" => {
                println!("Você tem certeza? (sim\\nao)");
                let confirm = match get_user_input() {
                    Ok(confirm) => confirm,
                    Err(err) => { println!("{}", err); continue }
                };

                if confirm.trim() == "sim" {
                    tc::reset_application();
                    pause();
                }
            },
            &_ => {
                println!("Digite uma opção viável!");
                pause();
                continue
            }
        }
    };
}