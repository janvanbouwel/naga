const IDENTIFIER = /[^ \s'$#()]+/

module.exports = grammar({
    name: "lang",

    word: $ => $.identifier,
    extras: $ => [],

    rules: {
        program: $ => seq(repeat($._ignore), optional($._program)),

        _program: $ => seq(
            $._expression,
            repeat(seq(repeat1($._ignore), $._expression)),
            repeat($._ignore)),

        _ignore: $ => choice($._ws, $.comment),
        _ws: $ => /\s/,
        comment: _ => /#.*/,

        _expression: $ => choice(
            $._quotable,
            $.function_definition
        ),

        _quotable: $ => choice(
            $.quote,
            $.function_binding,
            $._literal,
            $.identifier
        ),

        _literal: $ => choice(
            $.number,
            $.true,
            $.false
        ),

        quote: $ => seq("'", field("expression", optional($._quotable))),

        function_binding: $ => seq("$", field("identifier", $.identifier)),
        function_definition: $ => seq("(", seq(repeat($._ignore), field("body", alias(optional($._program), $.program))), ")"),

        number: _ => /\d+\.?\d*/,
        true: _ => prec(1, "TRUE"),
        false: _ => prec(1, "FALSE"),


        identifier: $ => IDENTIFIER,
    }
})
module.exports.IDENTIFIER = IDENTIFIER
