#!/bin/bash
ls src/$1/*.rs | xargs sed -i -e 's/AoristConcept, /AoristConcept, AoristRef, WrappedConcept, /g' -e 's/use paste::paste;/use paste::paste;\nuse std::fmt::Debug;/g' -e 's/\([A-Z][A-Za-z]\+\)(\1)/\1(AoristRef<\1>)/g'

