pub fn send_instructions(device_id: u32, task_name: &str, task_instruction_keyword: &str) -> String {
    let result = format!(
        "Device Id: {}, Task Name: {}, Task Instruction Keyword: {}",
        device_id, task_name, task_instruction_keyword
    );

    return result;
}

#[cfg(test)]
mod test {

    #[test]
    fn test_send_instructions() {
        //arrange
        let expected: &str =
            "Device Id: 22, Task Name: Wash the dishes, Task Instruction Keyword: START";

        //act
        let actual = super::send_instructions(22, "Wash the dishes", "START");

        //assert
        assert_eq!(expected, actual);
    }
}
