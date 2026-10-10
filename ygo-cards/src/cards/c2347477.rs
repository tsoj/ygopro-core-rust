//! Micro Coder
use super::prelude::*;

pub const ID: u32 = 2347477;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_CODE_TALKER, SET_CYNET],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Extra Material
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_code(d, e1, EFFECT_EXTRA_MATERIAL);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_operation(d, e1, Some(lua_fn(extracon)));
    effect::set_value_fn(d, e1, lua_fn(extraval));
    card::register_effect(d, c, e1, false);
    // if s.flagmap==nil then s.flagmap={} end; if s.flagmap[c]==nil then s.flagmap[c]={} end
    // (a missing list in the duel is the same as an empty one, see flagmap)
    // Search
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(thcon)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}

/// `s.flagmap[c]`: the flag effects this card's extra-material entries registered, kept in the
/// duel (key: the card, above the numbers the other script variables use).
fn flagmap_key(c: CardId) -> u64 {
    (1 << 32) | c.0 as u64
}
fn flagmap(d: &Duel, c: CardId) -> Vec<EffectId> {
    match s_var(d, ID, flagmap_key(c)) {
        Value::Multi(v) => v.iter().filter_map(|x| x.as_effect()).collect(),
        _ => Vec::new(),
    }
}
fn set_flagmap(d: &mut Duel, c: CardId, v: Vec<EffectId>) {
    set_s_var(d, ID, flagmap_key(c), Value::Multi(v.into_iter().map(Value::Effect).collect()));
}

fn extrafilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // c:IsLocation(LOCATION_MZONE) and c:IsControler(tp)
    card::is_location(d, c, LOCATION_MZONE) && card::is_controler(d, c, tp)
}
fn extracon(d: &mut Duel, a: Args) -> bool {
    // function s.extracon(c,e,tp,sg,mg,lc,og,chk)
    let e = a.e(1);
    let sg = a.g(3);
    let mg = a.g(4);
    let og = a.g(6);
    // return (sg+mg):Filter(s.extrafilter,nil,e:GetHandlerPlayer()):IsExists(Card.IsRace,1,og,RACE_CYBERSE) and
    let u = group::union(&sg, &mg);
    let hp = effect::get_handler_player(d, e);
    let f = group::filter(d, &u, &|d, c| extrafilter(d, c, hp), NO_EX);
    group::is_exists(d, &f, &|d, c| card::is_race(d, c, RACE_CYBERSE), 1, (&og).into())
        // sg:FilterCount(s.flagcheck,nil)<2
        && group::get_count(&group::filter(d, &sg, &|d, c| card::get_flag_effect(d, c, ID) > 0, NO_EX)) < 2
}
fn extraval(d: &mut Duel, a: Args) -> Value {
    // function s.extraval(chk,summon_type,e,...)
    let chk = a.int(0);
    let summon_type = a.int(1);
    let e = a.e(2);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    if chk == 0 {
        // local tp,sc=...
        let tp = a.p(3);
        let sc = a.card(4);
        // if summon_type~=SUMMON_TYPE_LINK or not sc:IsSetCard(SET_CODE_TALKER) or Duel.GetFlagEffect(tp,id)>0 then
        if summon_type != SUMMON_TYPE_LINK as i64 || !card::is_set_card(d, sc.nil(), &[SET_CODE_TALKER as u16]) || duel::get_flag_effect(d, tp, ID) > 0 {
            // return Group.CreateGroup()
            return Value::Group(group::create_group(&[]));
        }
        // table.insert(s.flagmap[c],c:RegisterFlagEffect(id,0,0,1))
        let fe = card::register_flag_effect(d, c, ID, 0, 0, 1, 0, 0);
        let mut list = flagmap(d, c);
        list.push(fe);
        set_flagmap(d, c, list);
        // return Group.FromCards(c)
        return Value::Group(group::create_group(&[c]));
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
    } else if chk == 2 {
        // for _,eff in ipairs(s.flagmap[c]) do eff:Reset() end
        for eff in flagmap(d, c) {
            effect::reset(d, eff);
        }
        // s.flagmap[c]={}
        set_flagmap(d, c, Vec::new());
    }
    Value::Nil
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // e:SetLabel(0)
    effect::set_label(d, a.e, &[0]);
    // if c:IsPreviousLocation(LOCATION_ONFIELD) then e:SetLabel(1) end
    if card::is_previous_location(d, c, LOCATION_ONFIELD) {
        effect::set_label(d, a.e, &[1]);
    }
    // return c:IsLocation(LOCATION_GRAVE) and c:IsPreviousLocation(LOCATION_ONFIELD|LOCATION_HAND) and r==REASON_LINK and c:GetReasonCard():IsSetCard(SET_CODE_TALKER)
    if !(card::is_location(d, c, LOCATION_GRAVE) && card::is_previous_location(d, c, LOCATION_ONFIELD | LOCATION_HAND) && a.r == REASON_LINK) {
        return false;
    }
    let rc = card::get_reason_card(d, c).nil();
    card::is_set_card(d, rc, &[SET_CODE_TALKER as u16])
}
fn thfilter(d: &mut Duel, c: CardId, chk: i64) -> bool {
    // ((c:IsSetCard(SET_CYNET) and c:IsSpellTrap()) or (chk==1 and c:IsRace(RACE_CYBERSE) and c:IsLevel(4))) and c:IsAbleToHand()
    ((card::is_set_card(d, c, &[SET_CYNET as u16]) && card::is_spell_trap(d, c)) || (chk == 1 && card::is_race(d, c, RACE_CYBERSE) && card::is_level(d, c, &[4])))
        && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,e:GetLabel())
        let label = effect::get_label(d, a.e);
        return duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c, label)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,e:GetLabel())
    let label = effect::get_label(d, a.e);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| thfilter(d, c, label)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,tp,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), Some(a.tp), REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
