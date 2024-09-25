
use crate::test_parse;

use super::*;

test_parse!(block1, 
    "{ print 1; print 2; print \"test\";}",
    "(block (print 1.0) (print 2.0) (print test))"
);

test_parse!(block2, 
    "{ print 1; print 2; \"test\"}",
    "(block (print 1.0) (print 2.0) test)"
);

test_parse!(block3, "{ }", "(block)");

test_parse!(block4, 
    " 
    {
        print 1;
        if (1 < 2) {
            print 2;
        }  
        print 3;
    }
    ",
    "(block (print 1.0) (if (< 1.0 2.0) (block (print 2.0))) (print 3.0))"
);

test_parse!(block5, 
    " 
    {
        print 1;
        if (1 < 2) {
            print 2;
        }  
        print 3;
        1
    }
    ",
    "(block (print 1.0) (if (< 1.0 2.0) (block (print 2.0))) (print 3.0) 1.0)"
);

test_parse!(block_incorrect_syntax1, ERROR, "{ print 1 print 2; 1 }");
test_parse!(block_incorrect_syntax2, ERROR, "{ print 1; print 2; 1 ");
test_parse!(block_incorrect_syntax3, ERROR, "{ print 1; print 2; 1;");
test_parse!(block_incorrect_syntax4, ERROR, "{ var test }");