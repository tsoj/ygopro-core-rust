//! Gold Pride - Nytro Head
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 59900655;
/// `local NYTRO_TOKEN=id+1`
pub const NYTRO_TOKEN: u32 = ID + 1;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[NYTRO_TOKEN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(_,tp) return Duel.GetLP(tp)<Duel.GetLP(1-tp) end)
    effect::set_condition(d, e1, Some(condition(|d, a| duel::get_lp(d, a.tp) < duel::get_lp(d, 1 - a.tp))));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Nytro Token" to the opponent's field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function (_,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e2, Some(condition(|d, a| duel::is_turn_player(d, 1 - a.tp))));
    // e2:SetTarget(s.tkntg)
    effect::set_target(d, e2, Some(cost_target(tkntg)));
    // e2:SetOperation(s.tknop)
    effect::set_operation(d, e2, Some(operation(tknop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy 1 "Nytro Token" and cards adjacent to it
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_MAIN_END)
    effect::set_hint_timing(d, e3, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(function (_,tp) return Duel.IsMainPhase() and Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e3, Some(condition(|d, a| duel::is_main_phase(d, None) && duel::is_turn_player(d, 1 - a.tp))));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.tkntg`: `Duel.IsPlayerCanSpecialSummonMonster(tp,NYTRO_TOKEN,0,TYPES_TOKEN,0,0,8,RACE_PYRO,ATTRIBUTE_FIRE,POS_FACEUP,1-tp)`
fn can_token(d: &mut Duel, tp: u8) -> bool {
    duel::is_player_can_special_summon_monster(
        d,
        tp,
        NYTRO_TOKEN,
        duel::MonsterData {
            setcodes: Some(vec![0]),
            type_: Some(TYPES_TOKEN),
            attack: Some(0),
            defense: Some(0),
            level: Some(8),
            race: Some(RACE_PYRO),
            attribute: Some(ATTRIBUTE_FIRE),
        },
        Some(POS_FACEUP as u8),
        Some(1 - tp),
        0,
    )
}
/// `s.tkntg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tkntg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp)>0 and Duel.IsPlayerCanSpecialSummonMonster(...) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), None, None).0 > 0 && can_token(d, tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
    true
}
/// `s.tknop(e,tp,eg,ep,ev,re,r,rp)`
fn tknop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0 and Duel.IsPlayerCanSpecialSummonMonster(...) then
    if duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && can_token(d, tp) {
        // local token=Duel.CreateToken(tp,NYTRO_TOKEN)
        let token = duel::create_token(d, tp, NYTRO_TOKEN).nil();
        // if Duel.SpecialSummonStep(token,0,tp,1-tp,false,false,POS_FACEUP) then
        if duel::special_summon_step(d, token, 0, tp, 1 - tp, false, false, POS_FACEUP as u8, None) {
            // Cannot be used as Link Material
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetDescription(3312)
            effect::set_description(d, e1, 3312);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_CANNOT_BE_LINK_MATERIAL)
            effect::set_code(d, e1, EFFECT_CANNOT_BE_LINK_MATERIAL);
            // e1:SetValue(1)
            effect::set_value(d, e1, 1);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // token:RegisterEffect(e1)
            card::register_effect(d, token, e1, false);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
/// `s.cfilter(c,tc,seq)`
fn cfilter(d: &mut Duel, c: CardId, tc: CardId, seq: u32) -> bool {
    if card::is_location(d, tc, LOCATION_SZONE) && card::is_controler(d, c, card::get_controler(d, tc)) {
        if card::is_location(d, c, LOCATION_MZONE) {
            return card::is_sequence(d, c, &[seq]);
        }
        return true;
    } else if card::is_location(d, tc, LOCATION_MZONE) {
        if card::is_location(d, c, LOCATION_SZONE) {
            return card::is_in_main_m_zone(d, tc, None)
                && group::is_contains(&card::get_column_group(d, tc, 0, 0), c)
                && card::is_controler(d, c, card::get_controler(d, tc));
        } else if card::is_location(d, c, LOCATION_MZONE) {
            if card::is_in_extra_m_zone(d, c, None) || card::is_in_extra_m_zone(d, tc, None) {
                return group::is_contains(&card::get_column_group(d, tc, 0, 0), c);
            } else {
                // c:IsSequence(seq-1,seq+1): seq-1 wraps for seq 0 (no such sequence, as in Lua's -1)
                return card::is_sequence(d, c, &[seq.wrapping_sub(1), seq + 1]) && card::is_controler(d, c, card::get_controler(d, tc));
            }
        }
    }
    false
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsFaceup() and chkc:IsCode(NYTRO_TOKEN) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_faceup(d, chkc) && card::is_code(d, chkc, &[NYTRO_TOKEN]);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsCode,NYTRO_TOKEN),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[NYTRO_TOKEN]))), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local tc=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsCode,NYTRO_TOKEN),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil):GetFirst()
    let sel = duel::select_target(d, tp, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[NYTRO_TOKEN]))), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    // local g=tc:GetColumnGroup(1,1):Filter(s.cfilter,nil,tc,tc:GetSequence())
    let seq = card::get_sequence(d, tc);
    let column = card::get_column_group(d, tc, 1, 1);
    let mut g = group::filter(d, &column, &|d, c| cfilter(d, c, tc, seq), NO_EX);
    // g:Merge(tc)
    group::add_card(&mut g, &CardSet::from_card(tc));
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:IsType(TYPE_TOKEN) then
    if card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc) && card::is_type(d, tc, TYPE_TOKEN) {
        // local g=tc:GetColumnGroup(1,1):Filter(s.cfilter,nil,tc,tc:GetSequence())
        let seq = card::get_sequence(d, tc);
        let column = card::get_column_group(d, tc, 1, 1);
        let mut g = group::filter(d, &column, &|d, c| cfilter(d, c, tc, seq), NO_EX);
        // g:Merge(tc)
        group::add_card(&mut g, &CardSet::from_card(tc));
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
