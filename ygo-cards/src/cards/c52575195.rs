//! Vivid Knight
//! ビビット騎士
use super::prelude::*;

pub const ID: u32 = 52575195;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish temporarily and Special Summon this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetLabel(0)
    effect::set_label(d, e1, &[0]);
    // e1:SetCondition(s.tgcon1)
    effect::set_condition(d, e1, Some(condition(tgcon1)));
    // e1:SetTarget(s.tgtg)
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    // e1:SetOperation(s.tgop)
    effect::set_operation(d, e1, Some(operation(tgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetLabel(1)
    effect::set_label(d, e2, &[1]);
    // e2:SetCondition(s.tgcon2)
    effect::set_condition(d, e2, Some(condition(tgcon2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.tgcon1(e,tp,eg,ep,ev,re,r,rp)`
fn tgcon1(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if rp==tp or not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return false end
    if a.rp == tp || !effect::is_has_property(d, a.re.nil(), EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]);
    let g = info[0].as_group().cloned();
    // if not g or #g~=1 then return false end
    let g = match g {
        Some(g) if g.len() == 1 => g,
        _ => return false,
    };
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g).nil();
    // e:SetLabelObject(tc)
    effect::set_label_object(d, a.e, Value::Card(tc));
    // return tc:IsControler(tp) and tc:IsLocation(LOCATION_MZONE) and tc:IsFaceup()
    //     and tc:IsAttribute(ATTRIBUTE_LIGHT) and tc:IsRace(RACE_BEASTWARRIOR)
    card::is_controler(d, tc, tp)
        && card::is_location(d, tc, LOCATION_MZONE)
        && card::is_faceup(d, tc)
        && card::is_attribute(d, tc, ATTRIBUTE_LIGHT)
        && card::is_race(d, tc, RACE_BEASTWARRIOR)
}
/// `s.tgcon2(e,tp,eg,ep,ev,re,r,rp)`
fn tgcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // if Duel.IsTurnPlayer(tp) then return false end
    if duel::is_turn_player(d, a.tp) {
        return false;
    }
    // local tc=Duel.GetAttackTarget()
    let tc = duel::get_attack_target(d);
    // e:SetLabelObject(tc)
    effect::set_label_object(d, a.e, match tc {
        Some(c) => Value::Card(c),
        None => Value::Nil,
    });
    // return tc and tc:IsFaceup() and tc:IsAttribute(ATTRIBUTE_LIGHT) and tc:IsRace(RACE_BEASTWARRIOR)
    match tc {
        Some(tc) => {
            card::is_faceup(d, tc) && card::is_attribute(d, tc, ATTRIBUTE_LIGHT) && card::is_race(d, tc, RACE_BEASTWARRIOR)
        }
        None => false,
    }
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card();
    // local label=e:GetLabel()
    let label = effect::get_label(d, a.e);
    // if chk==0 then return tc:IsAbleToRemove() and Duel.GetMZoneCount(tp,c)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and (label==0 or (label==1 and not c:IsStatus(STATUS_CHAINING))) end
    if a.chk == Some(0) {
        let tc = tc.nil();
        return card::is_able_to_remove(d, tc, None, None, None)
            && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None)
            && (label == 0 || (label == 1 && !card::is_status(d, c, STATUS_CHAINING)));
    }
    // Duel.SetTargetCard(tc)
    duel::set_target_card(d, tc.nil());
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, tc.map(CardSet::from_card), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, LOCATION_HAND as i32);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsControler(tp) and Duel.Remove(tc,tc:GetPosition(),REASON_EFFECT|REASON_TEMPORARY)>0 then
    if card::is_relate_to_effect(d, tc, a.e)
        && card::is_controler(d, tc, tp)
        && duel::remove(d, tc, card::get_position(d, tc) as u8, REASON_EFFECT | REASON_TEMPORARY, None, None) > 0
    {
        // Return it during your next Standby Phase
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
        effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
        // e1:SetRange(LOCATION_REMOVED)
        effect::set_range(d, e1, LOCATION_REMOVED);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // if Duel.IsTurnPlayer(tp) then
        let label = if duel::is_turn_player(d, tp) {
            // if Duel.IsPhase(PHASE_DRAW) then e1:SetLabel(Duel.GetTurnCount())
            if duel::is_phase(d, PHASE_DRAW) {
                duel::get_turn_count(d, None) as i64
            // else e1:SetLabel(Duel.GetTurnCount()+2) end
            } else {
                duel::get_turn_count(d, None) as i64 + 2
            }
        // else e1:SetLabel(Duel.GetTurnCount()+1) end
        } else {
            duel::get_turn_count(d, None) as i64 + 1
        };
        effect::set_label(d, e1, &[label]);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // e1:SetCondition(s.retcon)
        effect::set_condition(d, e1, Some(condition(retcon)));
        // e1:SetOperation(s.retop)
        effect::set_operation(d, e1, Some(operation(retop)));
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // if not c:IsRelateToEffect(e) then return end
        if !card::is_relate_to_effect(d, c, a.e) {
            return;
        }
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.retcon(e,tp,eg,ep,ev,re,r,rp)`
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetTurnCount()==e:GetLabel()
    duel::get_turn_count(d, None) as i64 == effect::get_label(d, a.e)
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // Duel.ReturnToField(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    duel::return_to_field(d, h, None, None);
    // e:Reset()
    effect::reset(d, a.e);
}
