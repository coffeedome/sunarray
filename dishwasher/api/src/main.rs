mod dishwasher_api;
mod send_instructions;

fn main() {
    send_instructions::send_instructions(22, "Wash the dishes", "START");

    dishwasher_api::dishwasher_api();
}
