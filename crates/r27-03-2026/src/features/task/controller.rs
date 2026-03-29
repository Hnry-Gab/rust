use super::model::TaskState;
use super::service as serv;

fn parse_id(id: String) -> Option<u32> {
    match id.trim().parse::<u32>() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("Digite um número válido!");
            None 
        }
    }
}

pub fn greet() {
    match serv::get_username() {
        Ok(username) => println!("{} qual operação deseja realizar?", username.trim()),
        Err(err) => println!("Erro ao adquirir o nome de usuário\nErro: {}", err)
    };
}

pub fn print_tasks() {
    let tasks = match serv::get_tasks() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Erro ao buscar tarefas\nError: {}", e);
            return;
        }
    };

    println!("Tarefas Atuais:\n");
    tasks.iter().for_each(|task| {
        println!("{}", task);
    });
}

pub fn print_task_by_id(id: String){
    let id = match parse_id(id) {
        Some(val) => val,
        None => return, 
    };
    
    match serv::get_task_by_id(id) {
        Ok(Some(task)) => println!("{}", task),
        Ok(None) => println!("Tarefa {} não encontrada.", id),
        Err(err) => eprintln!("Erro ao ler banco de dados: {}", err),
    }
}

pub fn create_task(description: String) {
    match serv::create_task(description) {
        Ok(task) => {
            println!("Tarefa {} adicionada com sucesso!\n{}", task.id, task);
        }
        Err(err) => {
            println!("Não foi possível adicionar a tarefa\nErro: {}", err);
        }
    }
}

pub fn delete_task(id: String) {
    let id = match parse_id(id) {
        Some(val) => val,
        None => return, 
    };

    match serv::delete_task(id) {
        Ok(result) => {
            if result {
                println!("Tarefa {} deletada com sucesso!", id);
            } else {
                println!("Tarefa {} inexistente", id);
            }
        }
        Err(err) => {
            println!("Não foi possível deletar a tarefa {}\nErro: {}", id, err);
        }
    }
}

pub fn edit_task(id: String, description: String) {
    let id = match parse_id(id) {
        Some(val) => val,
        None => return, 
    };

    match serv::edit_task(id, description) {
        Ok(_) => {
            println!("Tarefa {id} editada com sucesso!");
        }
        Err(err) => {
            println!("Não foi possível editar a tarefa {}\nErro: {}", id, err);
        }
    }
}

pub fn toggle_to_do(id: String) {
    let id = match parse_id(id) {
        Some(val) => val,
        None => return, 
    };

    match serv::change_state(id, TaskState::ToDo) {
        Ok(_) => {
            println!(
                "Tarefa {id} teve o status modificado para {}!",
                TaskState::ToDo
            );
        }
        Err(err) => {
            println!(
                "Não foi possível modificar o status da tarefa {}\nErro: {}",
                id, err
            );
        }
    }
}

pub fn toggle_doing(id: String) {
    let id = match parse_id(id) {
        Some(val) => val,
        None => return, 
    };
    
    match serv::change_state(id, TaskState::Doing) {
        Ok(_) => {
            println!(
                "Tarefa {id} teve o status modificado para {}!",
                TaskState::Doing
            );
        }
        Err(err) => {
            println!(
                "Não foi possível modificar o status da tarefa {}\nErro: {}",
                id, err
            );
        }
    }
}

pub fn toggle_done(id: String) {
    let id = match parse_id(id) {
        Some(val) => val,
        None => return, 
    };

    match serv::change_state(id, TaskState::Done) {
        Ok(_) => {
            println!(
                "Tarefa {id} teve o status modificado para {}!",
                TaskState::Done
            );
        }
        Err(err) => {
            println!(
                "Não foi possível modificar o status da tarefa {}\nErro: {}",
                id, err
            );
        }
    }
}

pub fn reset_application() {
    match serv::reset() {
        Ok(_) => println!("Aplicação resetada com sucesso, ação destrutiva sem backup!"),
        Err(err) => println!("Não foi possível resetar a aplicação\nErro: {}", err)
    }
}

pub fn change_username(username: String) {
    match serv::change_username(username) {
        Ok(_) => println!("Nome de usuário atualizado com sucesso!"),
        Err(err) => println!("Não foi possível atualizar o nome de usuário\nErro: {}", err)
    }
}