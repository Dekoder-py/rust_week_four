says 'Hello world'~

i = 1~
dowhile  i < 6:
  says i~
  i = i + 1~
end~

unless i == 6 then
  says "yes"~
else
  says "no"~
end~

says fetch("https://api.codingcorner.dev/health")~
