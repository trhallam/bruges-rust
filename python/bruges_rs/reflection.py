"""Reflection Module


"""

from .bruges_rspy import reflection as _librs


def critical_angles(vp1, vp2, vs2=None, degrees=True):

    ca1 = _librs.critical_angle(vp1, vp2)

    if vs2 is None:
        return ca1
    else:
        ca2 = _librs.critical_angle(vp1, vs2)
        return (ca1, ca2)
