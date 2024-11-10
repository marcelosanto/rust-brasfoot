use rand::seq::IteratorRandom;
use rand::Rng; 

#[derive(Debug, Clone)]
struct Player {
    name: String,
    pass_accuracy: u8,
    shot_accuracy: u8,
    foul_tendency: u8,
    luck: i8,
    yellow_cards: u8,
    red_card: bool,
}

impl Player {
    fn new(
        name: String,
        pass_accuracy: u8,
        shot_accuracy: u8,
        foul_tendency: u8,
        luck: i8,
    ) -> Self {
        Player {
            name,
            pass_accuracy,
            shot_accuracy,
            foul_tendency,
            luck,
            yellow_cards: 0,
            red_card: false,
        }
    }

    fn apply_yellow_card(&mut self) -> bool {
        self.yellow_cards += 1;
        if self.yellow_cards >= 2 {
            self.red_card = true;
        }
        self.red_card
    }
}

// Função para gerar um número aleatório com base na precisão do jogador
fn simulate_event(chance: u8, luck: i8) -> bool {
    let mut rng = rand::thread_rng();
    let adjusted_chance = (chance as i8 + luck) as u8;
    rng.gen_range(0..100) < adjusted_chance
}

fn simulate_pass(player: &Player) -> bool {
    simulate_event(player.pass_accuracy, player.luck)
}

fn simulate_shot(player: &Player) -> bool {
    simulate_event(player.shot_accuracy, player.luck)
}

fn simulate_foul(player: &Player) -> bool {
    simulate_event(player.foul_tendency, 0)
}

fn simulate_yellow_card() -> bool {
    rand::thread_rng().gen_bool(0.2)
}

fn simulate_corner() -> bool {
    rand::thread_rng().gen_bool(0.3)
}

fn simulate_offside() -> bool {
    rand::thread_rng().gen_bool(0.15)
}

fn log_event(message: &str) {
    println!("{}", message);
}

fn create_team(name_prefix: &str) -> Vec<Player> {
    let mut team = Vec::new();
    for i in 0..11 {
        team.push(Player::new(
            format!("{} Player {}", name_prefix, i + 1),
            rand::thread_rng().gen_range(60..91),
            rand::thread_rng().gen_range(60..86),
            rand::thread_rng().gen_range(5..16),
            rand::thread_rng().gen_range(-5..6),
        ));
    }
    team
}

fn main() {
    let mut team_a = create_team("Team A");
    let mut team_b = create_team("Team B");
    let home_advantage = 5; // Vantagem do time A jogar em casa

    let mut score_a = 0;
    let mut score_b = 0;
    let mut shots_on_goal_a = 0;
    let mut shots_on_goal_b = 0;
    let mut corners_a = 0;
    let mut corners_b = 0;
    let mut offsides_a = 0;
    let mut offsides_b = 0;
    let mut fouls_team_a = 0;
    let mut fouls_team_b = 0;
    let mut yellow_cards_team_a = 0;
    let mut yellow_cards_team_b = 0;
    let mut red_cards_team_a = 0;
    let mut red_cards_team_b = 0;

    // Simulação da partida (90 minutos)
    for minute in 0..90 {
        log_event(&format!("Minuto {}", minute + 1));

        // Determina qual time tem a posse de bola, considerando a vantagem em casa
        let possessing_team = if rand::thread_rng().gen_bool(0.55) {
            "A"
        } else {
            "B"
        };

        let (team, opponent_team) = if possessing_team == "A" {
            (&mut team_a, &mut team_b)
        } else {
            (&mut team_b, &mut team_a)
        };

        // Seleciona um jogador para a jogada, excluindo jogadores expulsos
        let player_with_ball = team
            .iter_mut()
            .filter(|p| !p.red_card)
            .choose(&mut rand::thread_rng())
            .unwrap();

        log_event(&format!("{} tocou a bola.", player_with_ball.name));

        // Simula o passe
        if simulate_pass(player_with_ball) {
            log_event(&format!(
                "{} fez um passe bem-sucedido.",
                player_with_ball.name
            ));

            // Simula o chute
            if simulate_shot(player_with_ball) {
                log_event(&format!("{} chutou para o gol!", player_with_ball.name));
                if possessing_team == "A" {
                    shots_on_goal_a += 1;
                    if rand::thread_rng().gen_bool(0.1) {
                        score_a += 1;
                        log_event("GOOOLLL do Time A!");
                    }
                } else {
                    shots_on_goal_b += 1;
                    if rand::thread_rng().gen_bool(0.1) {
                        score_b += 1;
                        log_event("GOOOLLL do Time B!");
                    }
                }
            } else {
                // Simula o escanteio
                if simulate_corner() {
                    log_event("Escanteio!");
                    if possessing_team == "A" {
                        corners_a += 1;
                    } else {
                        corners_b += 1;
                    }
                }
                // Simula o impedimento
                if simulate_offside() {
                    log_event("Impedimento!");
                    if possessing_team == "A" {
                        offsides_a += 1;
                    } else {
                        offsides_b += 1;
                    }
                }
            }
        }

        // Simula as faltas e cartões
        let fouling_player = opponent_team
            .iter_mut()
            .filter(|p| !p.red_card)
            .choose(&mut rand::thread_rng());
        if let Some(fouling_player) = fouling_player {
            if simulate_foul(fouling_player) {
                log_event(&format!("Falta cometida por {}", fouling_player.name));
                if possessing_team == "A" {
                    fouls_team_b += 1;
                    if simulate_yellow_card() {
                        yellow_cards_team_b += 1;
                        log_event(&format!(
                            "{} recebeu um cartão amarelo!",
                            fouling_player.name
                        ));
                        if fouling_player.apply_yellow_card() {
                            red_cards_team_b += 1;
                            log_event(&format!(
                                "{} foi expulso com cartão vermelho!",
                                fouling_player.name
                            ));
                        }
                    }
                } else {
                    fouls_team_a += 1;
                    if simulate_yellow_card() {
                        yellow_cards_team_a += 1;
                        log_event(&format!(
                            "{} recebeu um cartão amarelo!",
                            fouling_player.name
                        ));
                        if fouling_player.apply_yellow_card() {
                            red_cards_team_a += 1;
                            log_event(&format!(
                                "{} foi expulso com cartão vermelho!",
                                fouling_player.name
                            ));
                        }
                    }
                }
            }
        }
    }

    // Resultado final da partida
    log_event("Resultado Final:");
    log_event(&format!("Placar: Time A {} x {} Time B", score_a, score_b));
    log_event(&format!(
        "Chutes a gol: Time A {}, Time B {}",
        shots_on_goal_a, shots_on_goal_b
    ));
    log_event(&format!(
        "Escanteios: Time A {}, Time B {}",
        corners_a, corners_b
    ));
    log_event(&format!(
        "Impedimentos: Time A {}, Time B {}",
        offsides_a, offsides_b
    ));
    log_event(&format!(
        "Faltas: Time A {}, Time B {}",
        fouls_team_a, fouls_team_b
    ));
    log_event(&format!(
        "Cartões amarelos: Time A {}, Time B {}",
        yellow_cards_team_a, yellow_cards_team_b
    ));
    log_event(&format!(
        "Cartões vermelhos: Time A {}, Time B {}",
        red_cards_team_a, red_cards_team_b
    ));
}
