#[derive(Copy, Clone, Debug)]
enum Button {
    Right,
    Left,
    Up,
    Down,
    A,
    B,
    Select,
    Start,
}

struct Joypad {
    buttons: [bool; 8],           // true = pressionado
    select_directions: bool,      // Bit 4 (P14): seleciona direcionais
    select_buttons: bool,         // Bit 5 (P15): seleciona botões
    last_written_value: u8,       // Último valor escrito em 0xFF00
}

impl Joypad {
    fn new() -> Self {
        Self {
            buttons: [false; 8],
            select_directions: false,
            select_buttons: false,
            last_written_value: 0xFF,
        }
    }

    // Função que simula a leitura do registrador de entrada (0xFF00)
    fn read_joypad(&self) -> u8 {
        // Bits 4 e 5 são responsáveis pela seleção
        let mut result = 0xFF;

        // Mantém os bits 4 e 5 conforme o último valor escrito
        if self.select_directions {
            result &= !(1 << 4);  // bit 4 em 0 para indicar seleção
        } else {
            result |= 1 << 4;     // bit 4 em 1 para indicar não seleção
        }

        if self.select_buttons {
            result &= !(1 << 5);  // bit 5 em 0 para indicar seleção
        } else {
            result |= 1 << 5;     // bit 5 em 1 para indicar não seleção
        }

        // Se direcionais estão selecionados (P14 == 0)
        if self.select_directions {
            // Bits: 3 - Down, 2 - Up, 1 - Left, 0 - Right
            if self.buttons[Button::Down as usize] {
                result &= !(1 << 3);
            }
            if self.buttons[Button::Up as usize] {
                result &= !(1 << 2);
            }
            if self.buttons[Button::Left as usize] {
                result &= !(1 << 1);
            }
            if self.buttons[Button::Right as usize] {
                result &= !(1 << 0);
            }
        }

        // Se botões A/B/Select/Start estão selecionados (P15 == 0)
        if self.select_buttons {
            // Bits: 3 - Start, 2 - Select, 1 - B, 0 - A
            if self.buttons[Button::Start as usize] {
                result &= !(1 << 3);
            }
            if self.buttons[Button::Select as usize] {
                result &= !(1 << 2);
            }
            if self.buttons[Button::B as usize] {
                result &= !(1 << 1);
            }
            if self.buttons[Button::A as usize] {
                result &= !(1 << 0);
            }
        }

        result
    }

    // Função que simula a escrita no registrador de entrada (0xFF00)
    fn write_joypad(&mut self, value: u8) {
        self.last_written_value = value;
        self.select_buttons = value & (1 << 5) == 0;    // P15 selecionado se 0
        self.select_directions = value & (1 << 4) == 0; // P14 selecionado se 0
    }

    // Pressiona um botão
    fn press_button(&mut self, button: Button) {
        self.buttons[button as usize] = true;
    }

    // Solta um botão
    fn release_button(&mut self, button: Button) {
        self.buttons[button as usize] = false;
    }
}

// Para converter o enum em índices
impl Button {
    fn as_usize(&self) -> usize {
        match self {
            Button::Right => 0,
            Button::Left => 1,
            Button::Up => 2,
            Button::Down => 3,
            Button::A => 4,
            Button::B => 5,
            Button::Select => 6,
            Button::Start => 7,
        }
    }
}

impl From<Button> for usize {
    fn from(button: Button) -> Self {
        button.as_usize()
    }
}

fn main() {
    let mut joypad = Joypad::new();

    // Exemplo de uso
    joypad.write_joypad(0b00110000); // Nenhum grupo selecionado
    println!("Estado inicial: {:08b}", joypad.read_joypad());

    // Selecionar direcionais
    joypad.write_joypad(0b00100000); // P14 = 0, P15 = 1 -> Direcionais selecionados
    joypad.press_button(Button::Up);
    joypad.press_button(Button::Left);
    println!("Direcionais: {:08b}", joypad.read_joypad());

    // Selecionar botões
    joypad.write_joypad(0b00010000); // P14 = 1, P15 = 0 -> Botões selecionados
    joypad.press_button(Button::A);
    joypad.press_button(Button::Start);
    println!("Botões: {:08b}", joypad.read_joypad());

    // Soltar botão
    joypad.release_button(Button::A);
    println!("Botões após soltar A: {:08b}", joypad.read_joypad());
}
