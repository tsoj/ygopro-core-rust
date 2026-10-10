//! Code Generator
use super::prelude::*;

pub const ID: u32 = 30114823;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_CODE_TALKER],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If a Cyberse monster you control would be used as Link Material for a "Code Talker" monster, this card in your hand can also be used as material
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EFFECT_EXTRA_MATERIAL);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_operation(d, e1, Some(lua_fn(extracon)));
    effect::set_value_fn(d, e1, lua_fn(extraval));
    card::register_effect(d, c, e1, false);
    // if s.flagmap==nil then s.flagmap={} end; if s.flagmap[c]==nil then s.flagmap[c]={} end
    // (a missing list in the duel is the same as an empty one, see flagmap)
    // Send 1 Cyberse monster with 1200 or less ATK from your Deck to the GY, or, if this card on the field was used as material, you can add that card to your hand instead
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOGRAVE + CATEGORY_SEARCH + CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(tgcon)));
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    effect::set_operation(d, e2, Some(operation(tgop)));
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
        // sg:FilterCount(Card.HasFlagEffect,nil,id)<2
        && group::get_count(&group::filter(d, &sg, &|d, c| card::has_flag_effect(d, c, ID, None), NO_EX)) < 2
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
        // if summon_type~=SUMMON_TYPE_LINK or not sc:IsSetCard(SET_CODE_TALKER) or Duel.HasFlagEffect(tp,id) then
        if summon_type != SUMMON_TYPE_LINK as i64 || !card::is_set_card(d, sc.nil(), &[SET_CODE_TALKER as u16]) || duel::has_flag_effect(d, tp, ID, None) {
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
        // if summon_type&SUMMON_TYPE_LINK==SUMMON_TYPE_LINK and #sg>0 then
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
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsLocation(LOCATION_GRAVE) and c:IsPreviousLocation(LOCATION_HAND|LOCATION_ONFIELD) and r==REASON_LINK and c:GetReasonCard():IsSetCard(SET_CODE_TALKER)
    if !(card::is_location(d, c, LOCATION_GRAVE) && card::is_previous_location(d, c, LOCATION_HAND | LOCATION_ONFIELD) && a.r == REASON_LINK) {
        return false;
    }
    let rc = card::get_reason_card(d, c).nil();
    card::is_set_card(d, rc, &[SET_CODE_TALKER as u16])
}
fn tgfilter(d: &mut Duel, c: CardId, tohand_chk: i64) -> bool {
    // return c:IsRace(RACE_CYBERSE) and c:IsAttackBelow(1200) and (c:IsAbleToGrave() or (tohand_chk==1 and c:IsAbleToHand()))
    card::is_race(d, c, RACE_CYBERSE)
        && card::is_attack_below(d, c, 1200)
        && (card::is_able_to_grave(d, c, None) || (tohand_chk == 1 && card::is_able_to_hand(d, c, None)))
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local tohand_chk=e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD)
    let c = effect::get_handler(d, a.e).nil();
    let tohand_chk = card::is_previous_location(d, c, LOCATION_ONFIELD);
    if a.chk == Some(0) {
        // return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil,tohand_chk)
        // tohand_chk is a boolean here, and in Lua `true==1` is false: the filter gets 0.
        return duel::is_existing_matching_card(d, Some(&|d, c| tgfilter(d, c, 0)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // e:SetLabel(tohand_chk and 1 or 0)
    effect::set_label(d, a.e, &[if tohand_chk { 1 } else { 0 }]);
    if !tohand_chk {
        // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    } else {
        // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
        duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    }
    true
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local tohand_chk=e:GetLabel()==1
    let label = effect::get_label(d, a.e);
    let tohand_chk = label == 1;
    // local hint_msg=(tohand_chk and aux.Stringid(id,1) or HINTMSG_TOGRAVE)
    let hint_msg: u64 = if tohand_chk { aux::stringid(ID, 1) } else { HINTMSG_TOGRAVE as u64 };
    // Duel.Hint(HINT_SELECTMSG,tp,hint_msg)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, hint_msg);
    // local sc=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil,e:GetLabel()):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| tgfilter(d, c, label)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if not sc then return end
    let Some(sc) = sc else { return };
    if tohand_chk {
        // aux.ToHandOrElse(sc,tp)
        aux::to_hand_or_else(d, sc, a.tp, None, None, None);
    } else {
        // Duel.SendtoGrave(sc,REASON_EFFECT)
        duel::send_to_grave(d, sc, REASON_EFFECT, None, None);
    }
}
