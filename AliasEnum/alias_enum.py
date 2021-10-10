from enum import Enum
from types import DynamicClassAttribute
from typing import Iterable


class AliasEnum(Enum):
    """
    Derived from this class to define new enumerations with alias-based methods.

    Example:
        from datetime import date
        from typing import NamedTuple

        class Status(NamedTuple):
            height: float
            birthday: date

        # Right hand side of each member consists of aliases: Iterable[str] and value.
        # If collections.namedtuple, typing.NamedTuple or dataclasses.dataclass is set
        # as a value, each attributes of it is passed to memmber itself.
        class Friend(AliasEnum):
            TOM = (("Thomas", "Tom", "Tommy"),
                   Status(185.3, date(1992, 10, 17)))
            JIM = (("James", "Jim", "Jimmy"),
                   Status(173.4, date(1997, 7, 20)))

        # --- *.value returns the value (not the whole right hand side)
        print(Friend.TOM.value)
        # -> Status(height=185.3, birthday=datetime.date(1992, 10, 17))

        # --- *.aliases returns the aliases set
        print(Friend.JIM.aliases)  # -> {'James', 'Jimmy', 'Jim'} (random order)

        # --- *.height and *.birthday are available for each member
        print(Friend.TOM.height)  # -> 185.3
        print(Friend.JIM.birthday)  # -> 1997-07-28

        # --- Each member can be found by alias
        print(Friend.alias_to_member("Tommy") is Friend.TOM)  # -> True
        print(Friend.alias_to_member("James") is Friend.JIM)  # -> True

        # --- each value can get directly by the method 'alias_to_value'
        print(Friend.alias_to_value("Thomas").height)  # -> 185.3
        print(Friend.alias_to_value("Jimmy").birthday)  # -> 1997-07-28
        # Default behaviors when no alias matched can be defined by overriding
        # _default_member for alias_to_member, and _default_value for alias_to_value.
        # See the implementation of alias_to_member and alias_to_value.
    """

    def __new__(cls, aliases: Iterable[str], value):
        obj = object.__new__(cls)
        obj._value_ = value  # accessible by *.value
        obj._aliases_ = set(aliases)  # accessible by *.aliases

        # If collections.namedtuple, typing.NamedTuple or dataclasses.dataclass was
        # set as a value, each attribute is set to member itself.
        # Because "name", "value" and "aliases" has been already reserved,
        # AttributeError is raised if the value contains the attributes with those names.
        fields_attr = (
            "_fields",  # for collections.namedtuple, typing.NamedTuple
            "__dataclass_fields__",  # for dataclasses.dataclass
        )
        for attr in fields_attr:
            if hasattr(value, attr):
                for field in getattr(value, attr):
                    setattr(obj, field, getattr(value, field))

        # Mapping from alias to member is stored in cls._alias2member_map_
        # You can use this mapping by methods 'alias_to_member' and 'alias_to_value'.
        # You can also change the default behavior when no alias matched
        # by overriding methods '_default_member' or '_default_value' in a sub-class.
        if not hasattr(cls, "_alias2member_map_"):
            cls._alias2member_map_ = {}
        for alias in obj._aliases_:
            if alias in cls._alias2member_map_.keys():
                raise ValueError(f"duplicated alias '{alias}'")
            else:
                cls._alias2member_map_[alias] = obj
        return obj

    @DynamicClassAttribute
    def aliases(self):
        return self._aliases_

    @classmethod
    def alias_to_member(cls, alias: str):
        """Finds a member with an given alias"""
        filtered_alias = cls._alias_filter(alias)
        if filtered_alias in cls._alias2member_map_.keys():
            return cls._alias2member_map_[filtered_alias]
        else:
            return cls._default_member(alias, filtered_alias)

    @classmethod
    def _default_member(cls, alias: str, filtered_alias: str):
        """Define the the member to be returned when the input alias did not match.
        Override this method in a sub-class if necessary."""
        raise KeyError(filtered_alias)

    @classmethod
    def alias_to_value(cls, alias: str):
        """Finds a member with an given alias and returns its value"""
        filtered_alias = cls._alias_filter(alias)
        if filtered_alias in cls._alias2member_map_.keys():
            return cls._alias2member_map_[filtered_alias].value
        else:
            return cls._default_value(alias, filtered_alias)

    @classmethod
    def _default_value(cls, alias: str, filtered_alias: str):
        """Define the value to be returned when the input alias did not match.
        Override this method in a sub-class if necessary."""
        raise KeyError(filtered_alias)

    @classmethod
    def _alias_filter(cls, alias: str):
        """
        Converts input value before search for aliases. Override it
        in a sub-class if necessary.
        """
        return alias
