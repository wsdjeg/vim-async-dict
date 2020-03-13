#=============================================================================
# FILE: look.py
# AUTHOR: Shougo Matsushita <Shougo.Matsu at gmail.com>
#=============================================================================

from os.path import expanduser, expandvars
import re
import subprocess
from .base import Base

class Source(Base):
    def __init__(self, vim):
        Base.__init__(self, vim)

        self.name = 'minigrep'
        self.mark = '[D]'
        self.min_pattern_length = 2

        def get_look_var(shortname, default):
            name = 'async_{}'.format(shortname)
            if name not in vim.vars:
                return default
            return vim.vars[name]


        self.is_volatile = True
        self.words = None
        self.minigrep_executable = 'minigrep'
        self.dicts = get_look_var('dicts', None)
        if self.dicts:
            self.dicts = expandvars(expanduser(self.dicts))

    def _query_look(self, querystring):
        command = [self.minigrep_executable, querystring, self.dicts]

        if self.words is not None:
            command.append(self.words)

        return subprocess.check_output(command).splitlines()

    def gather_candidates(self, context):
        try:
            # We're adding :2 here to support head prefixed fuzzy search
            # "look" will return some results and deoplete will fuzzy search in
            # the rest.
            words = [
                x.decode(context['encoding'], errors='ignore')
                for x in self._query_look(context['complete_str'][:2])
            ]
        except subprocess.CalledProcessError:
            return []
        if re.match('[A-Z][a-z0-9_-]*$', context['complete_str']):
            words = [x[0].upper() + x[1:] for x in words]
        elif re.match('[A-Z][A-Z0-9_-]*$', context['complete_str']):
            words = [x.upper() for x in words]

        return [{ 'word': x } for x in words]
