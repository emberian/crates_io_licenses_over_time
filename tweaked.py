d = lambda s: [s.strip() for s in s.split('\n') if s.strip()]

good = d("""
MIT OR Apache-2.0
MIT OR Apache-2.0 OR BSD-2-Clause
MIT OR Apache-2.0 OR Beerware
MIT OR Apache-2.0 OR CC0-1.0
MIT OR Apache-2.0 OR GPL-3.0-or-later
MIT OR Apache-2.0+
MIT/Apache-2.0
MIT/Apache-2.0/BSD-3-Clause
MIT/Apache-2.0/CC0-1.0
MIT/Apache-2.0/ISC
MIT/Apache-2.0/LGPL-2.1
MIT/Apache-2.0/Unlicense
MIT/Apache-2.0/Unlicense/WTFPL
MIT/X11 OR Apache-2.0
MPL-2.0 OR Apache-2.0 OR MIT
MPL-2.0 OR MIT OR Apache-2.0
WTFPL OR MIT OR Apache-2.0
Apache-2.0 / MIT
Apache-2.0 / MIT / MPL-2.0
Apache-2.0/MIT
Apache-2.0/MIT/Unlicense
GPL-2.0/GPL-3.0/Apache-2.0/MIT
MPL-2.0/MIT/Apache-2.0
MPL-2.0 OR Apache-2.0 OR MIT
MPL-2.0 OR MIT OR Apache-2.0
CC0-1.0 OR MIT OR Apache-2.0
""")

lgpl = d("""
LGPL-2.0
LGPL-2.0+
LGPL-2.0-only
LGPL-2.0-or-later
LGPL-2.1
LGPL-2.1+
LGPL-2.1-only
LGPL-2.1-or-later
LGPL-3.0
LGPL-3.0+
LGPL-3.0-only
LGPL-3.0-or-later
LGPL-3.0/GPL-2.0/GPL-3.0
""")

# arbitrarily, I put the dual GPL/LGPL into the bucket that came first

gpl = d("""
GPL-1.0
GPL-2.0
GPL-2.0+
GPL-2.0-only
GPL-2.0-or-later
GPL-2.0/GPL-3.0
GPL-3.0
GPL-3.0+
GPL-3.0-only
GPL-3.0-or-later
GPL-3.0-or-later OR LGPL-3.0-or-later
GPL-3.0/GFDL-1.3
GPL-3.0/LGPL-3.0
""")

agpl = d("""
AGPL-1.0
AGPL-3.0
AGPL-3.0 WITH eCos-exception-2.0
AGPL-3.0+
AGPL-3.0-only
AGPL-3.0-or-later
""")

mpl = d("""
MPL-1.1
MPL-2.0
MPL-2.0+
MPL-2.0-no-copyleft-exception
""")


fsf = [lgpl, gpl, agpl]

collapse_into = []

collapse_into.extend((l, "Apache-2.0/MIT or superset") for l in good)
collapse_into.extend((l, "Some GPL") for l in gpl)
collapse_into.extend((l, "Some LGPL") for l in lgpl)
collapse_into.extend((l, "Some AGPL") for l in agpl)
collapse_into.extend((l, "Some MPL") for l in mpl)

collapse_into = dict(collapse_into)
