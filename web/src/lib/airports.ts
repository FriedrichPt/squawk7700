export interface Airport {
  icao: string;
  name: string;
  lat: number;
  lon: number;
  kind: "civil" | "military";
}

export const GERMAN_AIRPORTS: Airport[] = [
  // Major commercial airports
  { icao: "EDDF", name: "Frankfurt", lat: 50.0379, lon: 8.5622, kind: "civil" },
  { icao: "EDDM", name: "München", lat: 48.3537, lon: 11.7861, kind: "civil" },
  { icao: "EDDB", name: "Berlin Brandenburg", lat: 52.3667, lon: 13.5033, kind: "civil" },
  { icao: "EDDH", name: "Hamburg", lat: 53.6304, lon: 9.9882, kind: "civil" },
  { icao: "EDDS", name: "Stuttgart", lat: 48.6899, lon: 9.222, kind: "civil" },
  { icao: "EDDL", name: "Düsseldorf", lat: 51.2895, lon: 6.7668, kind: "civil" },
  { icao: "EDDK", name: "Köln/Bonn", lat: 50.8659, lon: 7.1427, kind: "civil" },
  { icao: "EDDV", name: "Hannover", lat: 52.4611, lon: 9.6851, kind: "civil" },
  { icao: "EDDN", name: "Nürnberg", lat: 49.4987, lon: 11.078, kind: "civil" },
  { icao: "EDDP", name: "Leipzig/Halle", lat: 51.4239, lon: 12.2364, kind: "civil" },
  { icao: "EDDE", name: "Erfurt-Weimar", lat: 50.9798, lon: 10.9581, kind: "civil" },
  { icao: "EDDR", name: "Saarbrücken", lat: 49.2146, lon: 7.1095, kind: "civil" },
  { icao: "EDDG", name: "Münster/Osnabrück", lat: 52.1346, lon: 7.6848, kind: "civil" },
  { icao: "EDDW", name: "Bremen", lat: 53.0475, lon: 8.7867, kind: "civil" },
  { icao: "EDDC", name: "Dresden", lat: 51.1328, lon: 13.7672, kind: "civil" },
  { icao: "EDLW", name: "Dortmund", lat: 51.5183, lon: 7.6122, kind: "civil" },
  { icao: "EDFH", name: "Frankfurt-Hahn", lat: 49.9487, lon: 7.2639, kind: "civil" },
  { icao: "EDLP", name: "Paderborn/Lippstadt", lat: 51.6141, lon: 8.6163, kind: "civil" },
  { icao: "EDNY", name: "Friedrichshafen", lat: 47.6713, lon: 9.5115, kind: "civil" },
  { icao: "EDAC", name: "Leipzig-Altenburg", lat: 50.9819, lon: 12.5063, kind: "civil" },
  { icao: "EDWQ", name: "Ganderkesee", lat: 53.0361, lon: 8.5031, kind: "civil" },
  { icao: "EDXN", name: "Nordholz-Spieka", lat: 53.7672, lon: 8.6553, kind: "civil" },

  // Bundeswehr / German military airbases
  { icao: "ETNT", name: "Wittmundhafen (JG 71)", lat: 53.5478, lon: 7.6675, kind: "military" },
  { icao: "ETNN", name: "Nörvenich (TLG 31)", lat: 50.8312, lon: 6.6585, kind: "military" },
  { icao: "ETSB", name: "Büchel (TLG 33)", lat: 50.1738, lon: 7.0633, kind: "military" },
  { icao: "ETSH", name: "Holzdorf", lat: 51.7679, lon: 13.1672, kind: "military" },
  { icao: "ETNS", name: "Schleswig-Jagel (TLG 51)", lat: 54.4593, lon: 9.5163, kind: "military" },
  { icao: "ETNW", name: "Wunstorf (LTG 62)", lat: 52.4571, lon: 9.4271, kind: "military" },
  { icao: "ETNL", name: "Rostock-Laage (TaktLwG 73)", lat: 53.9182, lon: 12.2783, kind: "military" },
  { icao: "ETSN", name: "Neuburg (TaktLwG 74)", lat: 48.7113, lon: 11.2115, kind: "military" },
  { icao: "ETHN", name: "Niederstetten", lat: 49.3911, lon: 9.9591, kind: "military" },
  { icao: "ETHF", name: "Fritzlar", lat: 51.1147, lon: 9.286, kind: "military" },
  { icao: "ETHS", name: "Faßberg", lat: 52.9195, lon: 10.1916, kind: "military" },
  { icao: "ETHB", name: "Bückeburg", lat: 52.2784, lon: 9.082, kind: "military" },
  { icao: "ETHR", name: "Roth", lat: 49.2173, lon: 11.1004, kind: "military" },
  { icao: "ETHL", name: "Laupheim", lat: 48.2202, lon: 9.91, kind: "military" },
  { icao: "ETSI", name: "Ingolstadt-Manching", lat: 48.7156, lon: 11.5341, kind: "military" },
  { icao: "ETSL", name: "Lechfeld", lat: 48.1854, lon: 10.8614, kind: "military" },
  { icao: "ETSF", name: "Kaufbeuren", lat: 47.8425, lon: 10.6256, kind: "military" },

  // US air bases in Germany (relevant for ADS-B tracking)
  { icao: "ETAR", name: "Ramstein AB", lat: 49.4369, lon: 7.6003, kind: "military" },
  { icao: "ETAD", name: "Spangdahlem AB", lat: 49.9727, lon: 6.6925, kind: "military" },
];
