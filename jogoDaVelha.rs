use std::io;
use std::io::Write;

fn main(){
    let mut tabuleiro: Vec<char> = Vec::new();
    iniciar_jogo(&mut tabuleiro);
}

fn iniciar_jogo(tabuleiro: &mut Vec<char>){   
    for _i in 0..9 {
        tabuleiro.push('.');
    }

    let mut count: i32 = 0;
    let mut jogador = 'X';

    loop {
        if count <= 8 {
            count += 1;
            imprime_tabuleiro(tabuleiro);
            if escolher_posicao(tabuleiro, &mut jogador) {
                break
                imprime_tabuleiro(tabuleiro);
            }
            
            if count % 2 == 0 {
                jogador = 'X';
            } else {
                jogador = 'O';
            }
        }
        else {
            break;
        }
    }
}

fn imprime_tabuleiro(tabuleiro: &Vec<char>) {
    for (i, &casa) in tabuleiro.iter().enumerate() {
        print!(" {} ", casa);

        if (i + 1) % 3 == 0 {
            println!();
        }
    }
}

fn escolher_posicao(tabuleiro: &mut Vec<char>, jogador: &mut char) -> bool {
    let mut buffer: String = String::new();
    print!("Digite a posição (1-9) para {:.}", jogador);
    println!();
    io::stdout().flush().expect("Erro ao atualizar stdout");
    io::stdin().read_line(&mut buffer).expect("Erro ao ler número.");
    let posicao: i32 = buffer.trim().parse().expect("Erro ao converter número");
    if tabuleiro[posicao as usize - 1] == '.' {
        tabuleiro[posicao as usize - 1] = *jogador;
    }
    else{
        print!("Posição já escolhida");
        println!();
    }
    if conferir_horizontal(tabuleiro, jogador) || (conferir_vertical(tabuleiro, jogador))||(conferir_diagonal(tabuleiro, jogador)) {
        print!("Jogador {:.} ganhou", jogador);
        println!();
        return true;
    }
    false

}

fn conferir_vertical(tabuleiro: &mut Vec<char>, jogador: &mut char) -> bool {
    for coluna in 0..3 {
        if (tabuleiro[coluna] == *jogador)
            && (tabuleiro[coluna + 3] == *jogador)
            && (tabuleiro[coluna + 6] == *jogador)
        {
            return true;
        }
    }
    false
}

fn conferir_horizontal(tabuleiro: &mut Vec<char>, jogador: &mut char) -> bool {
    for linha in 0..3 {
        let indice = linha * 3;
        if (tabuleiro[indice] == *jogador)
            && (tabuleiro[indice + 1] == *jogador)
            && (tabuleiro[indice + 2] == *jogador)
        {
            return true;
        }
    }
    false
}

fn conferir_diagonal(tabuleiro: &mut Vec<char>, jogador: &mut char) -> bool {
    if (tabuleiro[0] == *jogador) && (tabuleiro[4] == *jogador) && (tabuleiro[8] == *jogador) {
        return true;
    }

    if (tabuleiro[2] == *jogador) && (tabuleiro[4] == *jogador) && (tabuleiro[6] == *jogador) {
        return true;
    }
    false
}
