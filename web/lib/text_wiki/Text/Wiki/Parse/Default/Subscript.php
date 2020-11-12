<?php

/**
 *
 * Parses for subscripted text.
 *
 * @category Text
 *
 * @package Text_Wiki
 *
 * @author Paul M. Jones <pmjones@php.net>
 * @author Michal Frackowiak
 *
 * @license LGPL
 *
 * @version $Id$
 *
 */

/**
 *
 * Parses for subscripted text.
 *
 * @category Text
 *
 * @package Text_Wiki
 *
 * @author Paul M. Jones <pmjones@php.net>
 * @author Michal Frackowiak
 *
 */

class Text_Wiki_Parse_Subscript extends Text_Wiki_Parse {

    /**
     *
     * The regular expression used to parse the source text and find
     * matches conforming to this rule.  Used by the parse() method.
     *
     * @access public
     *
     * @var string
     *
     * @see parse()
     *
     */

    public $regex = '/' . 
                    ',,' . 
                    '([^\s](?:.*?[^\s])?)' .   # Match anything that does not start or end with whitespace
                    ',,' . 
                    '/x';

    /**
     *
     * Generates a replacement for the matched text.  Token options are:
     *
     * 'type' => ['start'|'end'] The starting or ending point of the
     * emphasized text.  The text itself is left in the source.
     *
     * @access public
     *
     * @param array &$matches The array of matches from parse().
     *
     * @return A pair of delimited tokens to be used as a placeholder in
     * the source text surrounding the text to be emphasized.
     *
     */

    function process(&$matches) {
        $start = $this->wiki->addToken($this->rule, array(
            'type' => 'start'));

        $end = $this->wiki->addToken($this->rule, array(
            'type' => 'end'));

        return $start . $matches[1] . $end;
    }
}