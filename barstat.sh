#!/usr/bin/env bash

DESKS=( ' one ' ' two ' ' three ' ' four ' ' five ' )

battery() {
  acpi --battery | \
  awk -F, '{print $2}'
}

dual_date() {
    (date; echo "   "; date -u; ) | tr -d '\n'
}

current_desk() {
  xprop -root  _NET_CURRENT_DESKTOP | \
  sed -e 's/_NET_CURRENT_DESKTOP(CARDINAL) = //'
}

desks() {
  local res;
  current_desk=$(current_desk)
  for index in ${!DESKS[*]}; do
    if [[ ${index} == ${current_desk} ]]; then
        desk="%{F#ffb58900}${DESKS[$index]}%{F-}"
    else
        desk="${DESKS[$index]}"
    fi
    res="${res}${desk}"
  done;
  echo -n "${res}"
}

while true; do
  echo "$(desks)%{c}%{F#FF859900}$(dual_date)%{F-}%{r}$(battery)"
  sleep 1;
done
