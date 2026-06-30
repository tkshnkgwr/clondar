let config = null;

// Built-in fallback config in case JSON load fails
const fallbackConfig = {
  fixed: {
    "01-01": "元日",
    "02-11": "建国記念の日",
    "02-23": "天皇誕生日",
    "04-29": "昭和の日",
    "05-03": "憲法記念日",
    "05-04": "みどりの日",
    "05-05": "こどもの日",
    "08-11": "山の日",
    "11-03": "文化の日",
    "11-23": "勤労感謝の日"
  },
  happy_mondays: [
    {
      "month": 1,
      "week": 2,
      "name": "成人の日",
      "start_year": 2000
    },
    {
      "month": 7,
      "week": 3,
      "name": "海の日",
      "start_year": 2003
    },
    {
      "month": 9,
      "week": 3,
      "name": "敬老の日",
      "start_year": 2003
    },
    {
      "month": 10,
      "week": 2,
      "name": "スポーツの日",
      "start_year": 2020
    }
  ],
  happy_mondays_legacy: [
    {
      "month": 1,
      "day": 15,
      "name": "成人の日",
      "start_year": 1949,
      "end_year": 1999
    },
    {
      "month": 7,
      "day": 20,
      "name": "海の日",
      "start_year": 1996,
      "end_year": 2002
    },
    {
      "month": 9,
      "day": 15,
      "name": "敬老の日",
      "start_year": 1966,
      "end_year": 2002
    },
    {
      "month": 10,
      "day": 10,
      "name": "体育の日",
      "start_year": 1966,
      "end_year": 1999
    },
    {
      "month": 10,
      "week": 2,
      "name": "体育の日",
      "start_year": 2000,
      "end_year": 2019
    }
  ],
  emperor_birthdays: [
    {
      "start_year": 1949,
      "end_year": 1988,
      "month": 4,
      "day": 29
    },
    {
      "start_year": 1989,
      "end_year": 2018,
      "month": 12,
      "day": 23
    },
    {
      "start_year": 2020,
      "month": 2,
      "day": 23
    }
  ],
  custom_overrides: {
    "2020": {
      "2020-07-23": "海の日",
      "2020-07-24": "スポーツの日",
      "2020-08-10": "山の日",
      "2020-07-20": null,
      "2020-08-11": null,
      "2020-10-12": null
    },
    "2021": {
      "2021-07-22": "海の日",
      "2021-07-23": "スポーツの日",
      "2021-08-08": "山の日",
      "2021-07-19": null,
      "2021-08-11": null,
      "2021-10-11": null
    }
  }
};

export async function initHolidays() {
  if (config) return config;
  try {
    const res = await fetch('/config/holidays.json');
    if (!res.ok) throw new Error('Failed to fetch holidays config');
    config = await res.json();
  } catch (e) {
    console.error('Error loading holidays config, using built-in fallback:', e);
    config = fallbackConfig;
  }
  return config;
}

export function getHolidays(yearParam) {
  const year = parseInt(yearParam);
  const holidays = {};
  
  if (year < 1948) return {};
  
  const activeConfig = config || fallbackConfig;

  const formatDate = (d) => `${d.getFullYear()}-${(d.getMonth() + 1).toString().padStart(2, '0')}-${d.getDate().toString().padStart(2, '0')}`;
  
  const getNthMonday = (y, m, n) => {
    const firstDay = new Date(y, m - 1, 1);
    let day = firstDay.getDay();
    let date = 1 + (day === 1 ? 0 : (8 - day) % 7) + (n - 1) * 7;
    return new Date(y, m - 1, date);
  };

  // 1. 固定祝日
  if (activeConfig.fixed) {
    Object.entries(activeConfig.fixed).forEach(([mmdd, name]) => {
      holidays[`${year}-${mmdd}`] = name;
    });
  }

  // 2. ハッピーマンデー (2000年以降)
  if (activeConfig.happy_mondays) {
    activeConfig.happy_mondays.forEach(rule => {
      if (year >= rule.start_year && (!rule.end_year || year <= rule.end_year)) {
        holidays[formatDate(getNthMonday(year, rule.month, rule.week))] = rule.name;
      }
    });
  }

  // 3. ハッピーマンデー旧制度 (〜1999年) および特定の固定日
  if (activeConfig.happy_mondays_legacy) {
    activeConfig.happy_mondays_legacy.forEach(rule => {
      if (year >= rule.start_year && year <= rule.end_year) {
        if (rule.day) {
          holidays[`${year}-${rule.month.toString().padStart(2, '0')}-${rule.day.toString().padStart(2, '0')}`] = rule.name;
        } else if (rule.week) {
          holidays[formatDate(getNthMonday(year, rule.month, rule.week))] = rule.name;
        }
      }
    });
  }

  // 4. 天皇誕生日
  if (activeConfig.emperor_birthdays) {
    activeConfig.emperor_birthdays.forEach(rule => {
      if (year >= rule.start_year && (!rule.end_year || year <= rule.end_year)) {
        holidays[`${year}-${rule.month.toString().padStart(2, '0')}-${rule.day.toString().padStart(2, '0')}`] = "天皇誕生日";
      }
    });
  }

  // 5. 天文計算による祝日
  const springEquinox = Math.floor(20.8431 + 0.242194 * (year - 1980) - Math.floor((year - 1980) / 4));
  holidays[`${year}-03-${springEquinox.toString().padStart(2, '0')}`] = "春分の日";
  const autumnEquinox = Math.floor(23.2488 + 0.242194 * (year - 1980) - Math.floor((year - 1980) / 4));
  holidays[`${year}-09-${autumnEquinox.toString().padStart(2, '0')}`] = "秋分の日";

  // 6. 振替休日 (1973年以降)
  if (year >= 1973) {
    const baseHolidays = Object.keys(holidays).sort();
    baseHolidays.forEach(dateStr => {
      const date = new Date(dateStr);
      if (date.getDay() === 0) {
        let substitute = new Date(date);
        substitute.setDate(substitute.getDate() + 1);
        while (holidays[formatDate(substitute)]) {
          substitute.setDate(substitute.getDate() + 1);
        }
        holidays[formatDate(substitute)] = "振替休日";
      }
    });
  }
  
  // 7. 国民の休日 (祝日に挟まれた平日, 1986年以降)
  if (year >= 1986) {
    const sortedKeys = Object.keys(holidays).sort();
    for (let i = 0; i < sortedKeys.length - 1; i++) {
      let d1 = new Date(sortedKeys[i]);
      let d2 = new Date(sortedKeys[i+1]);
      let diff = (d2 - d1) / (1000 * 60 * 60 * 24);
      if (diff === 2) {
        let bridgeDay = new Date(d1);
        bridgeDay.setDate(bridgeDay.getDate() + 1);
        if (bridgeDay.getDay() !== 0) {
          holidays[formatDate(bridgeDay)] = "国民の休日";
        }
      }
    }
  }

  // 8. 特定年の上書き (カスタムオーバーライド: オリンピック等)
  if (activeConfig.custom_overrides && activeConfig.custom_overrides[year]) {
    Object.entries(activeConfig.custom_overrides[year]).forEach(([dateStr, name]) => {
      if (name === null) {
        delete holidays[dateStr];
      } else {
        holidays[dateStr] = name;
      }
    });
  }

  return holidays;
}
