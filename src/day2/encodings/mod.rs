mod game;
mod instructions;
mod loss_draw_win;
mod natural;

use game::Round;
pub use instructions::{EncodedInstruction, EncodedOpponentMove, EncodedPlayerMove};
pub use loss_draw_win::LossDrawWinEncodingStrategy;
pub use natural::NaturalEncodingStrategy;

pub trait InstructionParsingStrategy {
    fn parse_encoded(&self, inst: &EncodedInstruction) -> Round;
}
