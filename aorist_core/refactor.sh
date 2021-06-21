#!/bin/bash
ls src/$1/*.rs | xargs sed -i \
  's/AoristConcept, /AoristConcept, AoristRef, WrappedConcept, /g'
ls src/$1/*.rs | xargs sed -i \
  's/use paste::paste;/use paste::paste;\nuse std::fmt::Debug;/g' 
ls src/$1/*.rs | xargs sed -i \
  's/\(\[constrainable\]\n[^\n]\+\)\([A-Z][A-Za-z, ]\+\)<\([A-Za-z0-9]\+\)>/\1\2<AoristRef<\3>>/g'
ls src/$1/*.rs | xargs sed -i \
  's/\(\[constrainable\]\n.\+: \)\([A-Z][A-Za-z0-9]\+\)/\1AoristRef<\2>/g' 
ls src/$1/*.rs | xargs sed -i \
  's/\([A-Z][A-Za-z]\+\)(\1)/\1(AoristRef<\1>)/g' 

