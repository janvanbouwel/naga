const IDENTIFIER = /[^ \s'$#]+/

module.exports = grammar({
    name: "lang",

    word: $ => $.identifier,
    extras: $ => [],

    rules: {
        program: $ => seq(repeat($._ignore), optional(seq(
            $._expression,
            repeat(seq(repeat1($._ignore), $._expression)),
            repeat($._ignore)))),

        _ignore: $ => choice($._ws, $.comment),
        _ws: $ => /\s/,
        comment: _ => /#.*/,

        _expression: $ => choice(
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

        quote: $ => seq("'", field("expression", optional($._expression))),

        function_binding: $ => seq("$", field("identifier", $.identifier)),

        number: _ => /\d+\.?\d*/,
        true: _ => prec(1, "TRUE"),
        false: _ => prec(1, "FALSE"),


        identifier: $ => IDENTIFIER,
    }
})
module.exports.IDENTIFIER = IDENTIFIER
