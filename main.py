#!/usr/bin/env python3
import argparse, sys
from colorama import Fore, Style
from modules import Gparams, verify_domains
from modules.color.color import colorize_logs
import modules.parser.parser as t
import os 
import subprocess
import signal

def banner():
    print(f"""\n +++ Nerium subdomains enumeration +++
                       {Fore.CYAN}DEF1NE INDUSTRIES{Style.RESET_ALL}  \n""")
    
def parse_args():
    lpath = os.path.dirname(os.path.abspath(__file__))
    
    parser = argparse.ArgumentParser(prog='subnerium', formatter_class=lambda prog: argparse.HelpFormatter(prog,max_help_position=407))
    parser.error = parser_error
    parser._optionals.title = "OPTIONS"
    parser.add_argument('-V', '--version', action='store_true', help='show nerium version')
    
    # Target Options
    target = parser.add_argument_group('TARGET')
    target.add_argument('-u', '--url', metavar='', help='target URL/host to scan')
    target.add_argument('-l', '--list', metavar='', help='target file to scan')

    # Template options
    optimizations = parser.add_argument_group("TEMPLATE")
    optimizations.add_argument('-t', '--templates',metavar='', default=f"/usr/share/subnerium/templates" ,help='list of template or template directory to run')
    
    # Optimizations
    optimizations = parser.add_argument_group("OPTIMIZATIONS")
    optimizations.add_argument('--timeout', metavar='TIME', default=30 ,help='time to wait in seconds before timeout')
    optimizations.add_argument('-c', '--concurrency', metavar='num', action='store', default=25,help='maximum number of templates to be executed in parallel')
    
    # Debug
    debug = parser.add_argument_group('DEBUG')
    debug.add_argument('--debug', action='store_true', help='Display errors and warnings')
    debug.add_argument('-v', '--verbose',action='store_true', help='show verbose output')

    # Output
    output = parser.add_argument_group("OUTPUT")
    output.add_argument('--silent', action='store_true', help='display findings only')
    output.add_argument('-o', '--output', action='store', help='output to the given filename.')
    
    return parser.parse_args()

def parser_error(errmsg):
    banner()
    print("Usage: nerium -u URL [Options] or use -h for help")
    print(f"{Fore.RED}Error:{Fore.RESET} {errmsg}")
    sys.exit()
    
def handler(signum, frame):
    try:
        # Coloque aqui o código que você deseja executar ao receber o sinal
        # ...
        print(f"{colorize_logs('info')} CTRL+C pressed: Exiting", signum)
        os._exit(0)
    except SystemExit:
        # Ignora a exceção SystemExit
        pass

def main() -> None:
    args = parse_args()
    
    args_dict = vars(args)  # Converte os argumentos para um dicionário
    signal.signal(signal.SIGINT, handler)

    if not args.url and not args.list:
        parser_error("Missing -u or --url or -l/--list option!")
        return
    
    if args.verbose and args.silent:
        parser_error("Verbose and silent flags are active")
    
    if not args.silent:
        banner()
    
    # Atualiza as configurações padrão com as configurações do usuário
    for key, value in args_dict.items():
        if value is not None:  # Ignora argumentos com valor None
            setattr(Gparams, key, value)

    Worker = t.ParserTemplates()
    if args.url:
        verify_domains.append(Gparams.url)
        Worker.RunnerTemplates()
    if args.list:
        if os.path.exists(Gparams.list) and os.path.isfile(Gparams.list):
            with open(Gparams.list, 'r') as target_file:
                for line in target_file:
                    verify_domains.append(line.rstrip('\n'))
                    Gparams.url = line
                    Worker.RunnerTemplates()
        else:
            parser_error("Can't open the specified file")
        
    if Gparams.verbose:
	    print(f"{colorize_logs('success')} Templates loaded for subdomain enumeration: {Worker.countTemplates(Gparams.templates, 1)}")
    #Worker.parseTemplate(f"{Gparams.templates}/subdomains/certspotter.yaml")
    #print(f"{colorize('critical')} {template.info.name}")  # irá imprimir "Low" com a cor verde


    
if __name__ == "__main__":
    main()
