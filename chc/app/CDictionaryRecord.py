# ------------------------------------------------------------------------------
# CodeHawk C Analyzer
# Author: Henny Sipma
# ------------------------------------------------------------------------------
# The MIT License (MIT)
#
# Copyright (c) 2017-2020 Kestrel Technology LLC
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.
# ------------------------------------------------------------------------------

from typing import cast, Callable, Dict, List, Tuple, Type, TypeVar, TYPE_CHECKING
import xml.etree.ElementTree as ET

import chc.util.IndexedTable as IT

if TYPE_CHECKING:
    from chc.app.CDeclarations import CDeclarations
    from chc.app.CDictionary import CDictionary


class CDictionaryRecord(IT.IndexedTableValue):
    """Base class for all objects kept in the CDictionary."""

    def __init__(
        self,
        cd: "CDictionary",
        index: int,
        tags: List[str],
        args: List[int],
    ) -> None:
        self.cd = cd
        self.index = index
        self.tags = tags
        self.args = args

    def get_key(self) -> Tuple[str, str]:
        return (",".join(self.tags), ",".join([str(x) for x in self.args]))

    def write_xml(self, node: ET.Element) -> None:
        (tagstr, argstr) = self.get_key()
        if len(tagstr) > 0:
            node.set("t", tagstr)
        if len(argstr) > 0:
            node.set("a", argstr)
        node.set("ix", str(self.index))


class CDeclarationsRecord(IT.IndexedTableValue):
    """Base class for all objects kept in the CDeclarations."""

    def __init__(
        self,
        decls: "CDeclarations",
        index: int,
        tags: List[str],
        args: List[int],
    ) -> None:
        self.decls = decls
        self.index = index
        self.tags = tags
        self.args = args

    def get_key(self) -> Tuple[str, str]:
        return (",".join(self.tags), ",".join([str(x) for x in self.args]))

    def get_dictionary(self) -> "CDictionary":
        return self.decls.dictionary

    def write_xml(self, node: ET.Element) -> None:
        (tagstr, argstr) = self.get_key()
        if len(tagstr) > 0:
            node.set("t", tagstr)
        if len(argstr) > 0:
            node.set("a", argstr)
        node.set("ix", str(self.index))


__c_dictionary_record_types: Dict[Tuple[type, str], Type[CDictionaryRecord]] = {}
CDiR = TypeVar('CDiR', bound=CDictionaryRecord, covariant=True)


def c_dictionary_record_tag(tag_name: str) -> Callable[[Type[CDiR]], Type[CDiR]]:
    def handler(t: Type[CDiR]) -> Type[CDiR]:
        __c_dictionary_record_types[(t.__bases__[0], tag_name)] = t
        return t
    return handler


def construct_c_dictionary_record(
    cd: "CDictionary",
    index: int,
    tags: List[str],
    args: List[int],
    superclass: Type[CDiR],
) -> CDiR:
    if (superclass, tags[0]) not in __c_dictionary_record_types:
        raise Exception("unknown type: " + tags[0])
    instance = __c_dictionary_record_types[(superclass, tags[0])](cd, index, tags, args)
    return cast(CDiR, instance)
