const { invoke } = window.__TAURI__.core;

const DAY_NAMES = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
const MONTH_NAMES = ['Jan','Feb','Mar','Apr','May','Jun','Jul','Aug','Sep','Oct','Nov','Dec'];

let weatherData = null;
let currentIndex = 0;
let currentPlace = '';

const form       = document.querySelector('#weather-form');
const placeInput = document.querySelector('#place-input');
const emptyState = document.querySelector('#empty-state');
const weatherCard = document.querySelector('#weather-card');
const statusMsg  = document.querySelector('#status-msg');
const navPrev    = document.querySelector('#nav-prev');
const navNext    = document.querySelector('#nav-next');
const navDays    = document.querySelector('#nav-days');

function formatTime(isoStr) {
  // "2026-03-12T06:09" → "06:09"
  return isoStr.split('T')[1] ?? isoStr;
}

function parseLocalDate(dateStr) {
  // "2026-03-12" → Date object interpreted in local timezone
  const [y, m, d] = dateStr.split('-').map(Number);
  return new Date(y, m - 1, d);
}

function isCurrentlyDay(forecast) {
  // Use is_day from the API for today's card; for other days derive from sunrise/sunset
  const now = new Date();
  const sunrise = new Date(forecast.sunrise);
  const sunset  = new Date(forecast.sunset);
  return now >= sunrise && now <= sunset;
}

function isDayForIndex(index) {
  if (index === 0) return weatherData.is_day === 1;
  const f = weatherData.forecast[index];
  const sunrise = new Date(f.sunrise);
  const sunset  = new Date(f.sunset);
  // approximate: if noon of that day is between sunrise and sunset → day
  const noon = parseLocalDate(f.time);
  noon.setHours(12);
  return noon >= sunrise && noon <= sunset;
}

function setDayNight(isDay) {
  if (isDay) {
    document.body.classList.add('is-day');
  } else {
    document.body.classList.remove('is-day');
  }
}

function renderDay(index, animate = false) {
  const f = weatherData.forecast[index];
  const date = parseLocalDate(f.time);
  const dayName = DAY_NAMES[date.getDay()];
  const monthName = MONTH_NAMES[date.getMonth()];
  const isToday = index === 0;

  const avgTemp = ((f.temperature_2m_max + f.temperature_2m_min) / 2).toFixed(1);

  const doRender = () => {
    document.getElementById('card-location').textContent = currentPlace;
    document.getElementById('card-date').textContent =
      `${isToday ? 'Today · ' : ''}${dayName}, ${date.getDate()} ${monthName} ${date.getFullYear()}`;
    document.getElementById('card-temp').textContent = avgTemp;
    document.getElementById('card-max').textContent = f.temperature_2m_max.toFixed(1);
    document.getElementById('card-min').textContent = f.temperature_2m_min.toFixed(1);
    document.getElementById('card-sunrise').textContent = formatTime(f.sunrise);
    document.getElementById('card-sunset').textContent = formatTime(f.sunset);
  };

  if (animate) {
    const main = document.querySelector('.card-main');
    main.classList.add('slide-exit');
    setTimeout(() => {
      main.classList.remove('slide-exit');
      doRender();
      main.classList.add('slide-enter');
      setTimeout(() => main.classList.remove('slide-enter'), 350);
    }, 220);
  } else {
    doRender();
  }

  // Day/night theme: index 0 uses real is_day; future days use noon heuristic
  const isDay = index === 0 ? weatherData.is_day === 1 : true;
  setDayNight(isDay);

  // Nav pills
  navPrev.disabled = index === 0;
  navNext.disabled = index === weatherData.forecast.length - 1;

  document.querySelectorAll('.nav-day-pill').forEach((pill, i) => {
    pill.classList.toggle('active', i === index);
  });
}

function buildNavPills() {
  navDays.innerHTML = '';
  weatherData.forecast.forEach((f, i) => {
    const date = parseLocalDate(f.time);
    const pill = document.createElement('div');
    pill.className = 'nav-day-pill' + (i === 0 ? ' active' : '');
    pill.textContent = i === 0 ? 'Today' : DAY_NAMES[date.getDay()];
    pill.addEventListener('click', () => {
      if (i !== currentIndex) {
        currentIndex = i;
        renderDay(currentIndex, true);
      }
    });
    navDays.appendChild(pill);
  });
}

function showWeather(data, place) {
  weatherData = data;
  currentIndex = 0;
  currentPlace = place;

  buildNavPills();
  renderDay(0, false);

  emptyState.style.display = 'none';
  weatherCard.style.display = 'flex';
  requestAnimationFrame(() => {
    requestAnimationFrame(() => weatherCard.classList.add('visible'));
  });
}

navPrev.addEventListener('click', () => {
  if (currentIndex > 0) {
    currentIndex--;
    renderDay(currentIndex, true);
  }
});

navNext.addEventListener('click', () => {
  if (weatherData && currentIndex < weatherData.forecast.length - 1) {
    currentIndex++;
    renderDay(currentIndex, true);
  }
});

form.addEventListener('submit', async (e) => {
  e.preventDefault();
  const place = placeInput.value.trim();
  if (!place) return;

  statusMsg.textContent = '';
  statusMsg.classList.add('loading-dots');
  statusMsg.textContent = 'Fetching weather';

  try {
    const data = await invoke('fetch_place', { place });
    statusMsg.classList.remove('loading-dots');
    statusMsg.textContent = '';
    showWeather(data, place);
  } catch (err) {
    statusMsg.classList.remove('loading-dots');
    statusMsg.textContent = 'Error: ' + err;
  }
});
