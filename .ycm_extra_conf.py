def Settings( **kwargs ):
  if kwargs['language'] == 'rust':
    return {
        'ls': {
            # https://github.com/rust-lang/rls#configuration
            'rust': {
                'all_features': True,
                'all_targets': False,
                'clippy_preference': "on",
            }
        }
    }
