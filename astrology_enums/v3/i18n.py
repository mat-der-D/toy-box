import json
import os
from enums import Language, Planet, Sign, Modality, Element

_I18N_ENUMS = Planet | Sign | Modality | Element

_DATA_DIR = os.path.join(os.path.dirname(__file__), 'i18n_data')


def _load_languages():
    lang_dict = {}
    for lang in Language:
        lang_path = os.path.join(_DATA_DIR, f'{lang.name.lower()}.json')
        with open(lang_path, encoding="utf8") as f:
            lang_dict[lang] = json.load(f)
    return lang_dict


_LANG_DICT = _load_languages()


def translate(lang: Language, variant: _I18N_ENUMS) -> str:
    return _LANG_DICT.get(lang, {}).get(variant.__class__.__name__, {}).get(variant.name, variant.name)
