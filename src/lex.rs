use std::fs::File;

//先初步只支持print("hello world")
#[derive(Debug)]
pub enum Token {
    Name(String) ,   //函数名print
    String(String) , //函数参数，也就是字符串hello world
    Eos,  //表示文件结束
}

#[derive(Debug)]
pub struct Lex {
    input : File,
}

impl Lex {
    pub fn new(input : File) -> Self; //基于输入文件创建语法分析器
    pub fn next(&mut self) -> Token; //返回下一个Token
}