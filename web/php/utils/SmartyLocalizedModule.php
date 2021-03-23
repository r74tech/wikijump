<?php

namespace Wikidot\Utils;

abstract use Ozone\Framework\SmartyModule;

class SmartyLocalizedModule extends SmartyModule
{

    public function render($runData)
    {

        $uu = $runData->getUser();
        if ($uu) {
            $lang = $uu->getLanguage();

            switch ($lang) {
                case 'pl':
                    $glang="pl_PL";
                    $wp = "pl";
                    break;
                case 'en':
                    $glang="en_US";
                    $wp = "www";
                    break;
            }

            putenv("LANG=$glang");
            putenv("LANGUAGE=$glang");
            setlocale(LC_ALL, $glang.'.UTF-8');
        }

        $out = parent::render($runData);

        if ($uu) {
            $lang = $GLOBALS['lang'];

            switch ($lang) {
                case 'pl':
                    $glang="pl_PL";
                    break;
                case 'en':
                    $glang="en_US";
                    break;
            }

            putenv("LANG=$glang");
            putenv("LANGUAGE=$glang");
            setlocale(LC_ALL, $glang.'.UTF-8');
        }

        return $out;
    }
}
