#!/bin/bash
ls src/$1/*.rs | xargs sed -i \
  -e 's/AoristConcept, /AoristConcept, AoristRef, WrappedConcept, /g' \
  -e 's/use paste::paste;/use paste::paste;\nuse std::fmt::Debug;/g' \
  -e 's/\(\[constrainable\]\n[^\n]\+\)\([A-Z][A-Za-z, ]\+\)<\([A-Za-z0-9]\+\)>/\1\2<AoristRef<\3>>/g'\
  -e 's/\(\[constrainable\]\n.\+: \)\([A-Z][A-Za-z0-9]\+\)/\1AoristRef<\2>/g'
  -e 's/\([A-Z][A-Za-z]\+\)(\1)/\1(AoristRef<\1>)/g' 

