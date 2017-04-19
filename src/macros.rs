/// Executes a sequence of pattern matching statements in order. If any match fails, the result is given as an `else`
/// expression. Otherwise the result is given as the `lift` expression.
#[macro_export]
macro_rules! lets {
    // Base case for pattern matching statements.
    (
        let $expect:pat = $result:expr;
        else $failure:expr;
        lift $success:expr;
    ) => {
        match $result {
            $expect => $success,
            _ => $failure,
        }
    };

    // Implicit else.
    (
        $(let $expect:pat = $result:expr;)+
        lift $success:expr;
    ) => {
        lets! {
            $(let $expect = $result;)*
            else ();
            lift $success;
        }
    };

    // Implicit lift.
    (
        $(let $expect:pat = $result:expr;)+
        else $failure:expr;
    ) => {
        lets! {
            $(let $expect = $result;)*
            else $failure;
            lift ();
        }
    };

    // Implicit else and lift.
    (
        $(let $expect:pat = $result:expr;)+
    ) => {
        lets! {
            $(let $expect = $result;)*
            else ();
            lift ();
        }
    };

    // Main case: multiple let statemens in a row.
    (
        let $expect_first:pat = $result_first:expr;
        $(let $expect_next:pat = $result_next:expr;)+
        else $failure:expr;
        lift $success:expr;
    ) => {
        match $result_first {
            $expect_first => lets! {
                $(let $expect_next = $result_next;)*
                else $failure;
                lift $success;
            },
            _ => $failure,
        }
    };
}
