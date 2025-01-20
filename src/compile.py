import lifelib
import sys

def load_and_compile_rules(*rules):
    
    rules = [lifelib.sanirule(r, drop_history=True) for r in rules]
    
    soname = lifelib.compile_rules(*rules)
    
    return soname

#main code:

args=sys.argv
args.pop(0)
sys.stdout.write(load_and_compile_rules(*args)+"\n") 
