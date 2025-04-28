# Projeto Paradigmas de Programacao
 ## Git FLow: 
 - Branch Develop: Serve para não enviarmos erros para o branch principal (Main)
 - Branch Feature: Quando for fazer uma nova funcionalidade/recurso criar uma Branch de Feature. Ex.: Feat_Classe_Carro.
 ## Listas de Comandos Git:
 - git add . -> envia para o fluxo de vercionamento (coloca ele na fila para enviar no repositorio).
 - git commit -a -m "Texto aqui" -> (m é para mandar msg  ) enviar para o repositorio na nuvem.
 - git push -> envia os arquivos para o git.
 - git pull -> recupera os arquivos modificados do git.
 - git checkout -b "nome da breach" -> cria uma brach.
 - git checkout main -> Muda para branch main.
 - git status -> saber em qual brainch vc está e quais arquivos estão modificados.
 ### Como utilizar os comandos:
 0. Atuliza a branch master/main: `git pull`
 1. Cria uma Branch `git checkout -b "Feat_funcionalidade"`
 2. Faça as alteracoes
 3. Após finalizar, `git add .` e depois `git commit -a -m "Comentario sobre a alteracao"`
 4. Faça um push da sua branch para o repositorio: `git push` ou caso seja a primeira vez `git push --set-upstream origin`
 5. Vá ao site do GitHub e abra a PR (Pull Request) ou avise sobre a alteração feita no grupo.

 ## Instalando RUST:
 - Para instalar o compilador da linguagem RUST entre no site [aqui](https://www.rust-lang.org/pt-BR/learn/get-started) e baixe o RUSTUP.
 - Tutorial [aqui](https://www.youtube.com/watch?v=2cji8a7CAds&list=PLWmXJQDlXOHX6UdAmXv6euoqDPUtMLpJf&index=2).
## Dependências:
 -  Pacotes utilizados: pixels = "0.13"
                        winit = "0.28"
                        log = "0.4"
                        env_logger = "0.11"
                        anyhow = "1.0"
## Como Rodar:
Comando para executar: `cargo run`.