from aorist import attributes as attr
from aorist import (
    RowStruct,
    AlluxioLocation,
    WebLocation,
    StaticHiveTableLayout,
    UpperSnakeCaseCSVHeader,
    ZipCompression,
    ORCEncoding,
    CSVEncoding,
    SingleFileLayout,
    RemoteStorage,
    HiveTableStorage,
    RemoteImportStorageSetup,
    StaticDataTable,
    default_tabular_schema,
    DataSet,
    ComputedFromLocalData,
    derive_integer_measure,
    attr_list,
)

# From: https://geonames.nga.mil/gns/html/gis_countryfiles.html
attributes = attr_list([
    # TODO: need richer attribute
    attr.NullableInt64(
        "rc",
        """
        Region Font Code.  A code that determines the equivalent characters
        used when generating the SORT and NO DIACRITICS (ND) feature names
        information rendered in Roman script.
        1 = Americas/Western Europe;
        2 = Eastern Europe;
        3 = Africa/Middle East;
        4 = Russia/ Central Asia;
        5 = Asia/Pacific;
        6 = Vietnam.
        """,
    ),
    attr.KeyInt64Identifier(
        "ufi",
        (
            "Unique Feature Identifier.  A number which "
            "uniquely identifies a feature."
        ),
    ),
    attr.Int64Identifier(
        "uni",
        """
        Unique Name Identifier.  A number which uniquely identifies a feature
        name.
        """,
    ),
    # TODO: need decimal degrees latlon
    attr.FloatLatitude(
        "lat",
        """
        Latitude of the feature in ± decimal degrees; DD; (± dd.dddddd):
        no sign (+) = North;
        negative sign (-) = South
        """,
    ),
    attr.FloatLongitude(
        "long",
        """
        Longitude of the feature in ± decimal degrees; DD; (± ddd.dddddd):
        no sign (+) = East;
        negative sign (-) = West.
        """,
    ),
    # TODO: need DMSH class
    attr.FloatLatitude(
        "dms_lat",
        """
        Latitude of the feature in degrees, minutes, seconds and
        hemisphere (DMSH) - dd:mm:ssH, where H is either N (North) or S (South)

        Note: In this format, DMSH values should be interpreted
        from left to right.
        """,
    ),
    attr.FloatLongitude(
        "dms_long",
        """
        Longitude of the feature in degrees, minutes, seconds and hemisphere
        (DMSH) - ddd:mm:ssH, where H is either E (East) or W (West)

        Note: In this format, DMSH values should be interpreted from
        left to right.
        """,
    ),
    attr.NullableStringIdentifier(
        "mgrs",
        """
        Military Grid Reference System coordinates. MGRS is an alpha-numeric
        system for expressing UTM/UPS coordinates. A single alpha-numeric
        value references an area that is unique for the entire earth.
        """,
    ),
    attr.NullableStringIdentifier(
        "jog",
        """
        Joint Operations Graphic reference.
        """,
    ),
    attr.NullableInt64(
        "fc",
        """
        Feature Class: Nine (9) major feature categories into which similar
        feature designations are grouped.
        A = Administrative region
        P = Populated place
        V = Vegetation
        L = Locality or area
        U = Undersea
        R = Streets, highways, roads, or railroad
        T = Hypsographic
        H = Hydrographic
        S = Spot
        """,
    ),
    attr.NullableStringIdentifier(
        "dsg",
        """
        Feature Designation Code.  A two to five-character code used to
        identify the type of feature a name is applied to. For a description of
        these codes/values, please see the "Look-up Tables..." section on the
        GNS Offered Services page.
        """,
    ),
    attr.NullableInt64(
        "pc",
        """
        Populated Place Class.  A numerical scale identifying the relative
        importance of a populated place. The scale ranges from 1 (high) to
        5 (low). The scale can also include NULL (no value) as a value for
        populated places with unknown or undetermined relative importance.
        """,
    ),
    attr.NullableStringIdentifier(
        "cc1",
        """
        Primary Geopolitical Code. A two alphabetic character code from the
        Geopolitical Entities and Codes (formerly FIPS 10-4 standard) that
        uniquely identifies a feature's primary geopolitical entity
        (countries, dependencies, andr areas of special sovereignty).

        This field can contain one, or multiple comma separated non-spaced
        country codes to identify international (shared) features that run
        through multiple countries for up to 255 characters (example:
        Danube River - AU,BU,EZ…).

        For a description of these codes/values, please see the "Look-up
        Tables..." section on the GNS Offered Services page.
        """,
    ),
    attr.NullableStringIdentifier(
        "adm1",
        """
        Primary administrative division code. A two character alpha-numeric
        code from the Geopolitical Entities and Codes (formerly FIPS 10-4
        standard) that describes the primary administrative division of a
        feature, similar to a state level in the United States. For a
        description of these codes/values, please see the "Look-up
        Tables..." section on the GNS Offered Services page.
        """,
    ),
    attr.Empty(
        "pop",
        """
        Population figures (no longer maintained; contains no values).
        """,
    ),
    attr.Empty(
        "elev",
        """
        Elevation in meters (no longer maintained; contains no values).
        """,
    ),
    attr.NullableStringIdentifier(
        "cc2",
        """
        Secondary Geopolitical Code. A two alphabetic character code from
        the Geopolitical Entities and Codes (formerly FIPS 10-4 standard)
        that uniquely identifies the particular feature name that is
        applicable in a particular geopolitical entity, or entities, for
        international features.

        This field can contain one, or multiple comma separated non-spaced
        country codes to identify international (shared) features that run
        through multiple countries for up to 255 characters (example: Donau
        – AU,GM… to identify the Austrian and German name for the Danube
        River).

        For a description of these codes/values, please see the "Look-up
        Tables..." section on the GNS Offered Services page.
        """,
    ),
    attr.NullableStringIdentifier(
        "nt",
        """
        Name Type:
        C = Conventional: A commonly used English-language name approved by the
        U.S. Board on Geographic Names (BGN) for use in addition to, or in lieu
        of, a BGN-approved local official name or names, e.g., Rome, Alps,
        Danube River.
        N = Approved: The BGN-approved local official name for a geographic
        feature. Except for countries with more than one official language;
        there is normally only one such name for a feature wholly within a
        country.
        D = Unverified: A name from a source whose official status cannot be
        verified by the BGN.
        P = Provisional: A geographic name of an area for which the territorial
        status is not finally determined or not recognized by the United
        States.

        VA = Anglicized variant: An English-language name that is derived by
        modifying the local official name to render it more accessible or
        meaningful to an English-language user.
        V = Variant: A former name, name in local usage, or other spelling
        found on various sources.

        NS = Non-Roman Script: The non-Roman script form of the BGN-approved
        local official name for a geographic feature. Except for countries with
        more than one official language; there is normally only one such name
        for a feature wholly within a country.
        DS = Unverified Non-Roman Script: The non-Roman script form of a name
        from a source whose official status cannot be verified by the BGN.
        VS = Variant Non-Roman Script: The non-Roman script form of a former
        name, name in local usage, or other spelling found on various sources.

        For a description of these codes/values, please see the "Look-up
        Tables..." section on the GNS Offered Services page.
        """,
    ),
    attr.NullableStringIdentifier(
        "lc",
        """
        Language Code. A three alphabetic character code (ISO 639-3)
        uniquely identifying the language assigned to a feature name. For a
        description of these codes/values, please see the "Look-up
        Tables..." section on the GNS Offered Services page.
        """,
    ),
    attr.NullableStringIdentifier(
        "short_form",
        """
        A part of the full name that could substitute for the full name.
        """,
    ),
    attr.NullableStringIdentifier(
        "generic",
        """
        The descriptive part of the full name such as Cerro (mountain), Arroyo
        (river), or Golfo (gulf) (generally does not apply to populated place
        names or English based generics).
        """,
    ),
    attr.NullableStringIdentifier(
        "sort_name_ro",
        """
        Sort name - reading order. A form of the full name that allows for
        alphabetical sorting of the file into gazetteer sequence. For Roman
        script names, all character/diacritic combinations and special
        characters are substituted with QWERTY (visible U.S. English keyboard)
        characters, all characters are upper-cased, numerals are converted to
        lower-case characters (0-9 = a-j), spaces and hyphens are stripped out,
        and commas replaced with a space. Additionally, for non-roman script
        names in languages using Arabic based letters. , vowel
        markings/pointers are removed. This field is included for the benefit
        of the end user of the data to aid in the sorting of names if required.
        """,
    ),
    attr.NullableStringIdentifier(
        "full_name_ro",
        """
        Full name - reading order. The full name is the complete name that
        identifies a named feature.  The full name is output in reading order,
        "Mount Everest", vs. reversed generic, "Everest, Mount".
        """,
    ),
    attr.NullableStringIdentifier(
        "full_name_nd_ro",
        """
        Full name - reading order with no diacritics. Same as the full name
        but the character/diacritic combinations and special characters are
        substituted with QWERTY (visible U.S. English keyboard) characters
        while still maintaining casing and spaces. This field also includes
        non-roman script based names which are stripped of vowel markings.
        """,
    ),
    attr.NullableStringIdentifier(
        "sort_name_rg",
        """
        Sort name - reversed generic. A form of the full name that allows
        for alphabetical sorting of the file into gazetteer sequence. For
        Roman script names, all character/diacritic combinations and
        special characters are substituted with QWERTY (visible U.S.
            English keyboard) characters, all characters are upper-cased,
        numerals are converted to lower-case characters (0-9 = a-j), spaces
        and hyphens are stripped out, and commas replaced with a space.
        Additionally, for non-roman script names in languages using Arabic
        based letters, vowel markings/pointers are removed. This field is
        included for the benefit of the end user of the data to aid in the
        sorting of names if required.
        """,
    ),
    attr.NullableStringIdentifier(
        "full_name_rg",
        """
        Full name - reversed generic. The full name is the complete name that
        identifies a named feature. The full name is output in reversed
        generic, "Everest, Mount" vs. reading order, "Mount Everest."
        """,
    ),
    attr.NullableStringIdentifier(
        "full_name_nd_rg",
        """
        Full name - reversed generic with no diacritics. Same as the full name
        but the character/diacritic combinations and special characters are
        substituted with QWERTY (visible U.S. English keyboard) characters
        while still maintaining casing and spaces. This field also includes
        non-roman script based names which are stripped of vowel markings.
        """,
    ),
    attr.NullableStringIdentifier(
        "note",
        """
        This field holds a geopolitical policy note concerning the feature,
        populated when CC1 and ADM1 are NULL, and it could contain multiple
        notes. If multiple notes are present, they will be delimited by
        semicolon followed by four spaces (1:note1; 2:note2; 3:note3;
        etc...).
        """,
    ),
    attr.NullableStringIdentifier(
        "modify_date",
        """
        The date a new feature was added or any part of an existing feature
        was modified (YYYY-MM-DD).
        """,
    ),
    attr.NullableStringIdentifier(
        "display",
        """
        Comma-separated non-spaced scale range values to aid in the
        visualization and selection of features. The display values range from
        1 to 9, and are listed as 1,2,3,4,5,6,7,8,9.

        The display values correspond to the following scale ranges:
        1 – 1 - 25,999
        2 – 26,000 - 67,999
        3 – 68,000 - 150,999
        4 – 151,000 - 225,999
        5 – 226,000 - 325,999
        6 – 326,000 - 425,999
        7 – 426,000 - 625,999
        8 – 626,000 - 999,999
        9 – 1,000,000 or Smaller

        Examples:
        A feature with display value of 1 should appear in a visualization
        client or service between scales of 1 and 25,999. It should not appear
        at scales of 26,000 and smaller.

        A feature with display values of 1,2,3 should appear in a visualization
        client or service between scales of 1 through 150,999. It should not
        appear at scales of 160,000 and smaller.

        A feature with display values of 1,2,3,4,5,6,7,8,9 should appear in a
        visualization client or service at all scales.
        """,
    ),
    # TODO: change to usize
    attr.NullableInt64(
        "name_rank",
        """
        A numeric value (1 - n) used to facilitate the display/visualization
        of the names associated with a feature.
        """,
    ),
    attr.NullableInt64(
        "uni2",
        """
        Unique Name Identifier (UNI) that links romanized names to their
        non-Roman script equivalent and vice-versa.
        """,
    ),
    attr.NullableString(
        "transl_cd",
        """
        Transliteration Code. This value indicates which transliteration
        system was used to arrive at the romanized equivalent of a non-Roman
        Script name.
        """,
    ),
    # TODO: need date field
    attr.NullableString(
        "nm_modify_date",
        """
        The date a new name was added or any part of an existing name was
        modified (YYYY-MM-DD).
        """,
    ),
    attr.NullableString(
        "f_efctv_dt",
        """
        Feature effectivity date. The documented date a feature came into
        existence (YYYY-MM-DD).
        """,
    ),
    attr.NullableString(
        "f_term_dt",
        """
        Feature termination date. The date a feature could no longer be
        verified on cartographic source or imagery (YYYY-MM-DD).
        """,
    ),
])
geonames_datum = RowStruct(
    name="geonames_datum",
    attributes=attributes,
)
remote = RemoteStorage(
    location=WebLocation(
        address=("https://geonames.nga.mil/gns/html/cntyfile/geonames_20210208.zip"),
    ),
    layout=SingleFileLayout(),
    encoding=CSVEncoding(
        compression=ZipCompression(
            filename="Countries.txt",
        ),
        header=UpperSnakeCaseCSVHeader(),
    ),
)
local = HiveTableStorage(
    location=AlluxioLocation(path="features"),
    layout=StaticHiveTableLayout(),
    encoding=ORCEncoding(),
)
geonames_table = StaticDataTable(
    name="features",
    schema=default_tabular_schema(geonames_datum),
    setup=RemoteImportStorageSetup(
        tmp_dir="geonames",
        remote=remote,
        local=[local],
    ),
    tag="geonames",
)
features_histogram_location = HiveTableStorage(
    location=AlluxioLocation(path="fc_hist"),
    layout=StaticHiveTableLayout(),
    encoding=ORCEncoding(),
)
geonames_dataset = DataSet(
    name="geonames-dataset",
    datumTemplates=[geonames_datum],
    assets=[geonames_table],
)
feature_class_count = derive_integer_measure(
    name="feature_class_frequency",
    comment="Frequency of Feature Class",
    attribute_names=["fc"],
    dataset=geonames_dataset,
    source_asset=geonames_table,
)
geonames_dataset.add_template(feature_class_count)
geonames_feature_class_histogram = StaticDataTable(
    name="feature_class_histogram",
    schema=default_tabular_schema(feature_class_count),
    setup=ComputedFromLocalData(
        source_asset_names={"features"},
        target=features_histogram_location,
        tmp_dir="/tmp/features_histogram",
    ),
    tag="fc_hist",
)
geonames_dataset.add_asset(geonames_feature_class_histogram)
