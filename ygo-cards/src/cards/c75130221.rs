//! Code Radiator
use super::prelude::*;

pub const ID: u32 = 75130221;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_CODE_TALKER],
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
    // disable
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_ATKCHANGE + CATEGORY_DISABLE);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(discon)));
    effect::set_target(d, e2, Some(cost_target(distg)));
    effect::set_operation(d, e2, Some(operation(disop)));
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
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
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
fn disfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and not (c:GetAttack()==0 and c:IsDisabled())
    card::is_faceup(d, c) && !(card::get_attack(d, c) == 0 && card::is_disabled(d, c))
}
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.disfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && disfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.disfilter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&disfilter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.disfilter,tp,0,LOCATION_MZONE,1,1+e:GetLabel(),nil)
    let label = effect::get_label(d, a.e);
    let _ = duel::select_target(d, a.tp, Some(&disfilter), a.tp, 0, LOCATION_MZONE, 1, (1 + label) as u16, false, NO_EX);
    true
}
fn disop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
            effect::set_value(d, e1, 0);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e1, false);
            // Duel.NegateRelatedChain(tc,RESET_TURN_SET)
            duel::negate_related_chain(d, tc, RESET_TURN_SET);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e2, EFFECT_DISABLE);
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e2, false);
            // local e3=Effect.CreateEffect(c)
            let e3 = effect::create_effect(d, c);
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e3, EFFECT_DISABLE_EFFECT);
            effect::set_value(d, e3, RESET_TURN_SET as i64);
            effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e3, false);
        }
    }
}
