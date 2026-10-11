//! Altergeist Pookuery
use super::prelude::*;

pub const ID: u32 = 59185998;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ALTERGEIST],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Extra Material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCode(EFFECT_EXTRA_MATERIAL)
    effect::set_code(d, e1, EFFECT_EXTRA_MATERIAL);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetOperation(s.extracon)
    effect::set_operation(d, e1, Some(lua_fn(extracon)));
    // e1:SetValue(s.extraval)
    effect::set_value_fn(d, e1, lua_fn(extraval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // if s.flagmap==nil then s.flagmap={} end
    // if s.flagmap[c]==nil then s.flagmap[c] = {} end
    if matches!(s_var(d, ID, flag_key(c)), Value::Nil) {
        set_flag_list(d, c, Vec::new());
    }
    // salvage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_TRIGGER_O+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1},EFFECT_COUNT_CODE_DUEL)
    effect::set_count_limit(d, e2, 1, ID, 1, EFFECT_COUNT_CODE_DUEL as u8);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// The Lua table `s.flagmap[c]` (the flag effects this card registered as material), kept in the duel.
fn flag_key(c: CardId) -> u64 {
    0x10000 + c.0 as u64
}
fn flag_list(d: &Duel, c: CardId) -> Vec<EffectId> {
    match s_var(d, ID, flag_key(c)) {
        Value::Multi(v) => v.iter().filter_map(|x| x.as_effect()).collect(),
        _ => Vec::new(),
    }
}
fn set_flag_list(d: &mut Duel, c: CardId, list: Vec<EffectId>) {
    set_s_var(d, ID, flag_key(c), Value::Multi(list.into_iter().map(Value::Effect).collect()));
}
/// `s.extrafilter(c,tp)`
fn extrafilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsLocation(LOCATION_MZONE) and c:IsControler(tp)
    card::is_location(d, c, LOCATION_MZONE) && card::is_controler(d, c, tp)
}
/// `s.extracon(c,e,tp,sg,mg,lc,og,chk)`
fn extracon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(1);
    let _tp = a.p(2);
    let sg = a.g(3);
    let mg = a.g(4);
    let og = a.g(6);
    // return (sg+mg):Filter(s.extrafilter,nil,e:GetHandlerPlayer()):IsExists(Card.IsSetCard,1,og,SET_ALTERGEIST)
    //     and sg:FilterCount(s.flagcheck,nil)<2
    let mut union = sg.clone();
    group::add_card(&mut union, &mg);
    let hp = effect::get_handler_player(d, e);
    let filtered = group::filter(d, &union, &|d, c| extrafilter(d, c, hp), NO_EX);
    let ex: Exclude = (&og).into();
    let exists = group::is_exists(d, &filtered, &|d, c| card::is_set_card(d, c, &[SET_ALTERGEIST as u16]), 1, ex);
    exists && group::filter_count(d, &sg, &|d, c| card::get_flag_effect(d, c, ID) > 0, NO_EX) < 2
}
/// `s.extraval(chk,summon_type,e,...)`
fn extraval(d: &mut Duel, a: Args) -> Value {
    let chk = a.int(0);
    let summon_type = a.int(1);
    let e = a.e(2);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    if chk == 0 {
        // local tp,sc=...
        let tp = a.p(3);
        let sc = a.c(4);
        // if summon_type~=SUMMON_TYPE_LINK or not sc:IsSetCard(SET_ALTERGEIST) or Duel.GetFlagEffect(tp,id)>0 then
        //     return Group.CreateGroup()
        if summon_type != SUMMON_TYPE_LINK as i64
            || !card::is_set_card(d, sc, &[SET_ALTERGEIST as u16])
            || duel::get_flag_effect(d, tp, ID) > 0
        {
            return Value::Group(group::create_group(&[]));
        }
        // table.insert(s.flagmap[c],c:RegisterFlagEffect(id,0,0,1))
        let eff = card::register_flag_effect(d, c, ID, 0, 0, 1, 0, 0);
        let mut list = flag_list(d, c);
        list.push(eff);
        set_flag_list(d, c, list);
        // return Group.FromCards(c)
        Value::Group(CardSet::from_card(c))
    } else if chk == 1 {
        // local sg,sc,tp=...
        let sg = a.g(3);
        let tp = a.p(5);
        // if summon_type&SUMMON_TYPE_LINK == SUMMON_TYPE_LINK and #sg>0 then
        if (summon_type & SUMMON_TYPE_LINK as i64) == SUMMON_TYPE_LINK as i64 && !sg.is_empty() {
            // Duel.Hint(HINT_CARD,tp,id)
            duel::hint(d, HINT_CARD as u8, tp, ID as u64);
            // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
            duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
        Value::Nil
    } else if chk == 2 {
        // for _,eff in ipairs(s.flagmap[c]) do eff:Reset() end
        for eff in flag_list(d, c) {
            effect::reset(d, eff);
        }
        // s.flagmap[c]={}
        set_flag_list(d, c, Vec::new());
        Value::Nil
    } else {
        Value::Nil
    }
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ALTERGEIST) and c:IsType(TYPE_LINK) and c:IsControler(tp) and c:IsLinkSummoned()
    card::is_set_card(d, c, &[SET_ALTERGEIST as u16]) && card::is_type(d, c, TYPE_LINK) && card::is_controler(d, c, tp) && card::is_link_summoned(d, c)
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, tp), 1, NO_EX)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsAbleToHand() end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(c), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,c)
        duel::confirm_cards(d, 1 - a.tp, CardSet::from_card(c));
    }
}
