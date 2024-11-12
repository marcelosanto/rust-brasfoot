use rand::seq::{IteratorRandom, SliceRandom};
use rand::Rng;

const CAMPO_LARGURA: i32 = 100;
const CAMPO_ALTURA: i32 = 50;

#[derive(Debug, Clone, Copy)]
struct Posicao {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Bola {
    posicao: Posicao,
}

impl Bola {
    fn new() -> Self {
        Bola {
            posicao: Posicao {
                x: CAMPO_LARGURA / 2,
                y: CAMPO_ALTURA / 2,
            },
        }
    }

    fn mover(&mut self, nova_posicao: Posicao) {
        self.posicao = nova_posicao;
    }
}

#[derive(Debug, Clone)]
struct Jogador {
    nome: String,
    posicao: Posicao,
    pass_accuracy: u8,
    dribble_skill: u8,
    shot_accuracy: u8,
    defense_skill: u8,
    foul_tendency: u8,
    luck: i8,
    yellow_cards: u8,
    red_card: bool,
}

impl Jogador {
    fn new(
        nome: String,
        pass_accuracy: u8,
        dribble_skill: u8,
        shot_accuracy: u8,
        defense_skill: u8,
        foul_tendency: u8,
        luck: i8,
    ) -> Self {
        Jogador {
            nome,
            posicao: Posicao {
                x: rand::thread_rng().gen_range(0..CAMPO_LARGURA),
                y: rand::thread_rng().gen_range(0..CAMPO_ALTURA),
            },
            pass_accuracy,
            dribble_skill,
            shot_accuracy,
            defense_skill,
            foul_tendency,
            luck,
            yellow_cards: 0,
            red_card: false,
        }
    }

    fn aplicar_cartao_amarelo(&mut self) -> bool {
        self.yellow_cards += 1;
        if self.yellow_cards >= 2 {
            self.red_card = true;
        }
        self.red_card
    }
}

// Goleiro com habilidades específicas de defesa
#[derive(Debug, Clone)]
struct Goleiro {
    nome: String,
    posicao: Posicao,
    save_skill: u8,
    luck: i8,
}

impl Goleiro {
    fn new(nome: String, save_skill: u8, luck: i8) -> Self {
        Goleiro {
            nome,
            posicao: Posicao {
                x: 0,
                y: CAMPO_ALTURA / 2,
            },
            save_skill,
            luck,
        }
    }

    // Simula a defesa do goleiro
    fn tentar_defender(&self) -> bool {
        simular_evento(self.save_skill, self.luck)
    }
}

// Funções para simulação dos eventos
fn simular_evento(chance: u8, luck: i8) -> bool {
    let mut rng = rand::thread_rng();
    let adjusted_chance = (chance as i8 + luck) as u8;
    rng.gen_range(0..100) < adjusted_chance
}

fn simular_passe(jogador: &Jogador) -> bool {
    simular_evento(jogador.pass_accuracy, jogador.luck)
}

fn simular_drible(jogador: &Jogador) -> bool {
    simular_evento(jogador.dribble_skill, jogador.luck)
}

fn simular_chute(jogador: &Jogador) -> bool {
    simular_evento(jogador.shot_accuracy, jogador.luck)
}

fn simular_interceptacao(defensor: &Jogador) -> bool {
    simular_evento(defensor.defense_skill, defensor.luck)
}

fn simular_falta(defensor: &Jogador) -> bool {
    simular_evento(defensor.foul_tendency, defensor.luck)
}

fn log_evento(mensagem: &str) {
    println!("{}", mensagem);
}

fn criar_time(nome_prefixo: &str) -> (Vec<Jogador>, Goleiro) {
    let mut time = Vec::new();
    for i in 0..10 {
        // 10 jogadores de linha
        time.push(Jogador::new(
            format!("{} Jogador {}", nome_prefixo, i + 1),
            rand::thread_rng().gen_range(60..91),
            rand::thread_rng().gen_range(50..81),
            rand::thread_rng().gen_range(60..86),
            rand::thread_rng().gen_range(50..85),
            rand::thread_rng().gen_range(5..16),
            rand::thread_rng().gen_range(-5..6),
        ));
    }
    let goleiro = Goleiro::new(
        format!("{} Goleiro", nome_prefixo),
        rand::thread_rng().gen_range(70..91),
        rand::thread_rng().gen_range(-5..6),
    );
    (time, goleiro)
}

fn verificar_saida_bola(bola: &Bola) -> Option<&'static str> {
    if bola.posicao.x < 0 || bola.posicao.x > CAMPO_LARGURA {
        Some("Lateral")
    } else if bola.posicao.y < 0 || bola.posicao.y > CAMPO_ALTURA {
        Some("Escanteio")
    } else {
        None
    }
}

fn main() {
    let (mut team_a, goleiro_a) = criar_time("Time A");
    let (mut team_b, goleiro_b) = criar_time("Time B");
    let mut bola = Bola::new();

    let mut score_a = 0;
    let mut score_b = 0;

    for minuto in 0..90 {
        log_evento(&format!("Minuto {}", minuto + 1));

        let posse = if rand::thread_rng().gen_bool(0.55) {
            "A"
        } else {
            "B"
        };
        let (time, outro_time, goleiro) = if posse == "A" {
            (&mut team_a, &mut team_b, &goleiro_b)
        } else {
            (&mut team_b, &mut team_a, &goleiro_a)
        };

        let jogador = time
            .iter_mut()
            .filter(|p| !p.red_card)
            .choose(&mut rand::thread_rng())
            .unwrap();
        let defensor = outro_time
            .iter_mut()
            .filter(|p| !p.red_card)
            .choose(&mut rand::thread_rng())
            .unwrap();

        log_evento(&format!("{} tenta uma jogada com a bola", jogador.nome));

        let acao_ataque = ["passar", "driblar", "chutar"]
            .choose(&mut rand::thread_rng())
            .unwrap();

        match *acao_ataque {
            "passar" => {
                log_evento(&format!("{} tenta passar a bola", jogador.nome));
                if simular_passe(jogador) {
                    if simular_interceptacao(defensor) {
                        log_evento(&format!("{} interceptou o passe!", defensor.nome));
                    } else {
                        log_evento(&format!("Passe bem-sucedido por {}", jogador.nome));
                    }
                } else {
                    log_evento(&format!(
                        "Passe de {} falhou e a bola saiu para lateral",
                        jogador.nome
                    ));
                    bola.mover(Posicao {
                        x: if rand::thread_rng().gen_bool(0.5) {
                            0
                        } else {
                            CAMPO_LARGURA - 1
                        },
                        y: rand::thread_rng().gen_range(0..CAMPO_ALTURA),
                    });
                    verificar_saida_bola(&bola);
                }
            }
            "driblar" => {
                log_evento(&format!(
                    "{} tenta driblar o defensor {}",
                    jogador.nome, defensor.nome
                ));
                if simular_drible(jogador) {
                    if simular_interceptacao(defensor) {
                        log_evento(&format!("{} interceptou o drible!", defensor.nome));
                    } else {
                        log_evento(&format!("Drible bem-sucedido por {}", jogador.nome));
                    }
                } else {
                    log_evento(&format!(
                        "Drible falhou e o defensor {} interceptou",
                        defensor.nome
                    ));
                }
            }
            "chutar" => {
                log_evento(&format!("{} tenta um chute a gol", jogador.nome));
                if simular_chute(jogador) {
                    if goleiro.tentar_defender() {
                        log_evento(&format!("{} fez a defesa!", goleiro.nome));
                        if rand::thread_rng().gen_bool(0.5) {
                            log_evento("A bola foi desviada para escanteio!");
                        }
                    } else {
                        log_evento(&format!("GOOOLLL de {}!", jogador.nome));
                        if posse == "A" {
                            score_a += 1;
                        } else {
                            score_b += 1;
                        }
                    }
                } else {
                    log_evento("Chute foi para fora!");
                }
            }
            _ => {}
        }
    }

    log_evento("Resultado Final:");
    log_evento(&format!("Placar: Time A {} x {} Time B", score_a, score_b));
}
