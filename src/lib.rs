use std::collections::HashMap;

use rand::Rng;
use structures::desenvolvedor::Desenvolvedor;

mod structures {
    pub mod desenvolvedor;
    pub mod slot_semanal;
}

fn main() {
    let mut desenvolvedores: HashMap<String, Desenvolvedor> = HashMap::new();

    // Adicionando desenvolvedores
    let desenvolvedor1 = Desenvolvedor::new("Ana".to_string());
    let desenvolvedor2 = Desenvolvedor::new("Jose".to_string());

    desenvolvedores.insert(desenvolvedor1.nome.clone(), desenvolvedor1);
    desenvolvedores.insert(desenvolvedor2.nome.clone(), desenvolvedor2);

    // Função para gerar slots aleatórios
    fn gerar_slots_aleatorios(quantidade: usize) -> Vec<structures::slot_semanal::SlotSemanal> {
        let mut rng = rand::rng();
        let mut slots = Vec::new();

        let mut dia_atual = rng.random_range(1..=5);
        let mut hora_atual = rng.random_range(9..=16);

        for _ in 0..quantidade {
            let duracao = rng.random_range(1..=3); // Duração de 1 a 3 horas
            let hora_fim = hora_atual + duracao;

            slots.push(structures::slot_semanal::SlotSemanal {
                dia_semana: dia_atual,
                hora_inicio: hora_atual,
                hora_fim: hora_fim.min(18), // Não ultrapassar 18h
            });

            // Decidir se o próximo slot será consecutivo (30% de chance)
            if rng.random_bool(0.3) {
                hora_atual = hora_fim;
                // Se passou das 18h, vai para o próximo dia
                if hora_atual >= 18 {
                    dia_atual = if dia_atual >= 5 { 1 } else { dia_atual + 1 };
                    hora_atual = 9;
                }
            } else {
                // Slot não consecutivo
                dia_atual = rng.random_range(1..=5);
                hora_atual = rng.random_range(9..=16);
            }
        }

        slots
    }

    // Adicionando slots para ana
    if let Some(alice) = desenvolvedores.get_mut("ana") {
        let slots_ana = gerar_slots_aleatorios(10);
        for slot in slots_ana {
            ana.adicionar_slot(slot);
        }
    }

    // Adicionando slots para jose
    if let Some(jose) = desenvolvedores.get_mut("jose") {
        let slots_jose = gerar_slots_aleatorios(8);
        for slot in slots_jose {
            jose.adicionar_slot(slot);
        }
    }

    // Encontra o desenvolvedor com mais horas interruptas, mostrando o nome e as horas
    let mut max_horas = 0;
    let mut desenvolvedor_max_horas = String::new();

    let mut max_slots = 0;
    let mut desenvolvedor_max_slots = String::new();
    for (nome, desenvolvedor) in &desenvolvedores {
        let (_, slots) = desenvolvedor.horas_interruptas_trabalhadas();
        if slots > max_slots {
            max_slots = slots;
            desenvolvedor_max_slots = nome.clone();
        }
    }
    for (nome, desenvolvedor) in &desenvolvedores {
        let (horas, _) = desenvolvedor.horas_interruptas_trabalhadas();
        if horas > max_horas {
            max_horas = horas;
            desenvolvedor_max_horas = nome.clone();
        }
    }

    println!(
        "Desenvolvedor com mais horas interruptas: {} com {} horas",
        desenvolvedor_max_horas, max_horas
    );
    println!(
        "Desenvolvedor com mais slots: {} com {} slots",
        desenvolvedor_max_slots, max_slots
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structures::slot_semanal::SlotSemanal;

    #[test]
    fn test_encontrar_desenvolvedor_com_mais_horas() {
        let mut desenvolvedores = HashMap::new();

        // Criar um desenvolvedor com muitas horas interruptas
        let mut ana = Desenvolvedor::new("ana".to_string());
        ana.adicionar_slot(SlotSemanal {
            dia_semana: 1,
            hora_inicio: 9,
            hora_fim: 12,
        });
        ana.adicionar_slot(SlotSemanal {
            dia_semana: 1,
            hora_inicio: 12,
            hora_fim: 15,
        });
        ana.adicionar_slot(SlotSemanal {
            dia_semana: 1,
            hora_inicio: 15,
            hora_fim: 18,
        });

        // Criar um desenvolvedor com menos horas interruptas
        let mut jose = Desenvolvedor::new("Jose".to_string());
        jose.adicionar_slot(SlotSemanal {
            dia_semana: 2,
            hora_inicio: 10,
            hora_fim: 12,
        });
        jose.adicionar_slot(SlotSemanal {
            dia_semana: 3,
            hora_inicio: 13,
            hora_fim: 15,
        });

        desenvolvedores.insert(ana.nome.clone(), ana);
        desenvolvedores.insert(jose.nome.clone(), jose);

        // Verificar se ana tem mais horas interruptas
        let mut max_horas = 0;
        let mut desenvolvedor_max_horas = String::new();

        for (nome, desenvolvedor) in &desenvolvedores {
            let (horas, _) = desenvolvedor.horas_interruptas_trabalhadas();
            if horas > max_horas {
                max_horas = horas;
                desenvolvedor_max_horas = nome.clone();
            }
        }

        assert_eq!(desenvolvedor_max_horas, "ana");
        assert_eq!(max_horas, 9); // 9 horas consecutivas (9 às 18)
    }

    #[test]
    fn test_encontrar_desenvolvedor_com_mais_slots() {
        let mut desenvolvedores = HashMap::new();

        // Criar um desenvolvedor com vários slots consecutivos
        let mut ana = Desenvolvedor::new("ana".to_string());
        ana.adicionar_slot(SlotSemanal {
            dia_semana: 1,
            hora_inicio: 9,
            hora_fim: 10,
        });
        ana.adicionar_slot(SlotSemanal {
            dia_semana: 1,
            hora_inicio: 10,
            hora_fim: 11,
        });
        ana.adicionar_slot(SlotSemanal {
            dia_semana: 1,
            hora_inicio: 11,
            hora_fim: 12,
        });
        ana.adicionar_slot(SlotSemanal {
            dia_semana: 1,
            hora_inicio: 12,
            hora_fim: 13,
        });

        // Criar um desenvolvedor com menos slots consecutivos
        let mut jose = Desenvolvedor::new("jose".to_string());
        jose.adicionar_slot(SlotSemanal {
            dia_semana: 2,
            hora_inicio: 10,
            hora_fim: 11,
        });
        jose.adicionar_slot(SlotSemanal {
            dia_semana: 2,
            hora_inicio: 11,
            hora_fim: 12,
        });

        desenvolvedores.insert(ana.nome.clone(), ana);
        desenvolvedores.insert(jose.nome.clone(), jose);

        // Verificar se ana tem mais slots consecutivos
        let mut max_slots = 0;
        let mut desenvolvedor_max_slots = String::new();

        for (nome, desenvolvedor) in &desenvolvedores {
            let (_, slots) = desenvolvedor.horas_interruptas_trabalhadas();
            if slots > max_slots {
                max_slots = slots;
                desenvolvedor_max_slots = nome.clone();
            }
        }

        assert_eq!(desenvolvedor_max_slots, "ana");
        assert_eq!(max_slots, 4); // 4 slots consecutivos
    }
}