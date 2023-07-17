import yaml
import os
import concurrent.futures
from pathlib import Path

from modules.temp.templates import Templates
from typing import Dict, Union, Any, List
from modules import Gparams
from modules.protocols.request import requestHTTP
from modules.color.color import colorize_logs

class ParserTemplates():
    def countTemplates(self, directory, op:int):
        templates = []
        for root, directories, files in os.walk(directory):
            for file in files:
                if file.endswith('.yaml'):
                    path_template = os.path.join(root, file)
                    try:
                        with open(path_template) as f:
                            yaml.safe_load(f)
                            templates.append(path_template)
                    except Exception:
                        pass
        if op:
            return len(templates)
        
        return templates

    def RunnerTemplates(self):
        templates = self.countTemplates(Gparams.templates, 0)
        try:
            threads_max = int(Gparams.concurrency)

            with concurrent.futures.ThreadPoolExecutor(max_workers=threads_max) as executor:
                # Adiciona as tarefas (execução de cada template) ao pool
                futures = [executor.submit(self.executeTemplate, template) for template in templates]

                # Espera todas as tarefas terminarem
                for future in concurrent.futures.as_completed(futures):
                    # Pega o resultado da tarefa, se houver
                    result = future.result()
        except Exception as e:
            print(f"{colorize_logs('error')} {e}")

    def executeTemplate(self, template):
        ret_template = self.parseTemplate(template)
        if not ret_template:
            return
             
        requestHTTP().executeRequest(ret_template)
    
    def parseTemplate(self, template):
        template_dict = self.openTemplate(template)
        mvars = {
            "domain": Gparams.url,
            "token": parserAPIKeys(id=template_dict['id'])
        }
        template_dict = self.substitute_variables(template_dict, mvars)
        template = Templates(**template_dict)
        if mvars['token'] == "":
            if Gparams.debug:
                print(f"{colorize_logs('warning')} {template.info.name} - Missing API Token")
                return None
                
        return template
        
    def openTemplate(self, template: str) -> Dict[str, any]:
        """
        Abre um arquivo YAML contendo um template e retorna um dicionário Python correspondente.
        
        :param template: O caminho do arquivo YAML contendo o template a ser aberto.
        :type template: str
        
        :return: Um dicionário Python contendo as informações do template.
        :rtype: Dict[str, any]

        """
        with open(template, 'r') as f:
            return yaml.safe_load(f)

    def substitute_variables(self, d: Union[Dict[str, Any], List[Any]], substitutions: Dict[str, Any]) -> Union[Dict[str, Any], List[Any]]:
        """
        Substitui todas as variáveis em uma estrutura de dicionário ou lista recursivamente.
        As variáveis devem estar no formato '{{nome_da_variável}}'.

        :param d: A estrutura de dicionário ou lista onde as variáveis serão substituídas.
        :param substitutions: O dicionário de substituições, onde a chave é o nome da variável e o valor é o valor a ser substituído.
        :return: A estrutura de dicionário ou lista com as variáveis substituídas.
        """
        if isinstance(d, dict):
            return {
                k: self.substitute_variables(v, substitutions) 
                for k, v in d.items()
            }
        elif isinstance(d, list):
            return [
                self.substitute_variables(e, substitutions) 
                for e in d
            ]
        elif isinstance(d, str):
            for k, v in substitutions.items():
                d = d.replace(f"{{{{{k}}}}}", str(v))
            return d
        else:
            return d

def parserAPIKeys(id: str):

    api_keys = ParserTemplates().openTemplate(f"{Gparams.local_home}/.config/subnerium/apikeys.yaml")
    try:
        return api_keys[id]
    except KeyError:
        return None
