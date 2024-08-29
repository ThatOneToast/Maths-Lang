/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: "maths_lang_grammar",

  rules: {
    source_file: $ => repeat($.statement),

    statement: $ => choice(
      $.variable_assignment,
      $.function_call,
      $.conditional_statement,
      $.loop_statement,
      $.print_statement,
      $.break_statement,
      $.continue_statement
    ),

    variable_assignment: $ => seq(
      'let',
      $.identifier,
      '=',
      $.expression,
      ';'
    ),

    function_call: $ => seq(
      '@',
      $.identifier,
      '(',
      optional(seq($.expression, repeat(seq(',', $.expression)))),
      ')',
      ';'
    ),

    conditional_statement: $ => seq(
      'if',
      $.expression,
      '{',
      repeat($.statement),
      '}',
      optional(seq('else', '{', repeat($.statement), '}'))
    ),

    loop_statement: $ => seq(
      choice('loop', 'LOOP'),
      $.expression,
      repeat($.statement),
      'loop_end'
    ),

    break_statement: $ => 'break',

    continue_statement: $ => 'continue',

    print_statement: $ => seq(
      ';',
      $.identifier,
      ';'
    ),

    expression: $ => choice(
      $.number,
      $.identifier,
      prec.left(seq($.expression, choice('+', '-', '*', '/', '^'), $.expression))
    ),

    identifier: $ => /[a-zA-Z_][a-zA-Z0-9_]*/,
    number: $ => /[0-9]+(\.[0-9]*)?/
  },
});
