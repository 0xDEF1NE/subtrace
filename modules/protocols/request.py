import requests, json
from urllib.parse import urlparse
from colorama import Fore, Style
import re

from modules.templates.templates import Templates
from modules.color.color import colorize_logs
from modules import Gparams, verify_domains

class requestHTTP():
    def __init__(self):
        self.ofile = None
        
    def verifyStatus(self, response, name, status):
        value = status[0]

        if value.status:
            for status_code in value.status:
                if status_code == response.status_code:
                    req = json.loads(response.text)
                    text = self.parsingReturnJsonRequestByWords(req, False, value.words)
                    if Gparams.debug:
                        print(f"{colorize_logs('warning')} {name} - {text}")
                    return 1
            return 0
                    
    def executeRequest(self, template: Templates):
        info = template.info
        path = template.requests[0].path
        headers = template.requests[0].headers
        matchers = template.requests[0].matchers
        settings = template.settings
        if not Gparams.silent:
        	print(f"{colorize_logs('info')} Running Template: {info.name}")
            
        status = [a for a in matchers if a.type == 'status']

        if Gparams.output:
            self.ofile = open(Gparams.output, 'a+')
        try:
            with requests.get(path, headers=headers, timeout=int(Gparams.timeout)) as r:

                if status:
                    if self.verifyStatus(r, info.name, status) != 0:
                        return
                for matcher in matchers:
                    if matcher.part == "body":
                        if matcher.type == "word":
                            response = json.loads(r.text)
                            result = self.parsingReturnJsonRequestByWords(response, settings, matcher.words)
                            
                        elif matcher.type == "indice":
                            response = json.loads(r.text)
                            result = self.parsingReturnJsonRequestByIndice(response, matcher.indice)
                        
                        elif matcher.type == "regex":
                            result = self.parsingReturnBodyByRegex(r.text, matcher.value, info.name)
                
                for i in info.tags:
                    if i == 'subenum':
                        for i in result:
                            self.verifyDomainExists(info.name, i)
                if Gparams.output:
                    self.ofile.close()

        except Exception as e:
            if Gparams.debug:
                print(f"{colorize_logs('error')} {info.name} - {e}")
            if Gparams.output:
                self.ofile.close()
            return
    
    def verifyDomainExists(self, function, domain):
        # Remove '*.' do começo dos dominios.
        domain = domain.lstrip("*.")
        # Verifica se o dominio já foi encontrado, se não foi, a função abaixo insere na lista.
        if not domain or domain in verify_domains:
            return None
        
        verify_domains.append(domain)
        if Gparams.verbose:
            print(f"{Fore.CYAN}{function}{Style.RESET_ALL}: {domain}")
            return
        if Gparams.output:
            self.ofile.write(domain + '\n')
        print(domain)
        
    
    def parsingReturnBodyByRegex(self, response, regex_value, name):
        resp = response
        try:
            return re.findall(regex_value, resp)
        except Exception as e:
            print(f"{colorize_logs('error')} {name} - {e}")
            
    def parsingReturnJsonRequestByIndice(self, json_ret, indice):
        value = json_ret
        if indice:
            new_value = []
            for i in value:
                new_value.append(self.parsingURLreturnDomain(i[indice]))
        
        return new_value
        
    def parsingReturnJsonRequestByWords(self, json_ret, settings, words):
        def remove_n(string):
            string = string.split("\n")[0]
            return string
        value = json_ret
        
        if words:
            for key in words:
                if isinstance(value, list):
                    tmp = []
                    for item in value:
                        item_value = self.parsingReturnJsonRequestByWords(item, settings, [key])
                        if item_value:
                            if isinstance(item_value, list):
                                for i in item_value:
                                    if "\n" in i:
                                        tmp.append(remove_n(i))
                                    else:
                                        tmp.append(i)
                            else:
                                if "\n" in item_value:
                                    tmp.append(remove_n(item_value))
                                else:
                                    tmp.append(item_value)
                    value = tmp
                elif isinstance(value, dict):
                    value = value.get(key, {})
                                            
                else:
                    value = {}

        if settings and settings.concatenate:
            return self.concatenateDomain(value)
        
        return value
    
    def parsingURLreturnDomain(self, url):
        parsed = urlparse(url)
        return parsed.netloc.split(":")[0]
    
    def concatenateDomain(self, subdomains):
        new_value = []
        for key in subdomains:
            new_value.append(f"{key}.{Gparams.url}")
        return new_value
