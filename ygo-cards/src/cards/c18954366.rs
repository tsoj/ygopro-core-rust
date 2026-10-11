//! Necroquip Prism
//! Scripted by Naim
use super::prelude::*;

pub const ID: u32 = 18954366;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 monster from hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_EQUIP + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsMonster() and c:IsLevelBelow(4) and not c:IsForbidden()
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp,c:GetLevel())
    if !(card::is_monster(d, c) && card::is_level_below(d, c, 4) && !card::is_forbidden(d, c)) {
        return false;
    }
    let lv = card::get_level(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, lv)), tp, LOCATION_HAND, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,lv)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: u32) -> bool {
    // return c:IsMonster() and not c:IsForbidden() and c:IsLevelBelow(lv) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_monster(d, c)
        && !card::is_forbidden(d, c)
        && card::is_level_below(d, c, lv)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && filter(d, chkc, a.e, tp);
    }
    // local ft1=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // local ft2=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let mut ft2 = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // if not e:GetHandler():IsLocation(LOCATION_SZONE) then ft2=ft2-1 end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_location(d, h, LOCATION_SZONE) {
        ft2 -= 1;
    }
    // if chk==0 then return ft1>0 and ft2>0 and Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return ft1 > 0 && ft2 > 0 && duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // e:SetLabel(g:GetFirst():GetLevel())
    let first = group::get_first(d, &g).nil();
    let lv = card::get_level(d, first);
    effect::set_label(d, a.e, &[lv as i64]);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,g,1,0,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(g), 1, 0, LOCATION_GRAVE as i32);
    true
}
/// `s.eqlimit(e,c)`
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c==e:GetLabelObject()
    Some(c) == effect::get_label_object(d, e).as_card()
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // return re:GetHandler():IsCode(e:GetLabel())
    let re = a.e(1);
    let rh = effect::get_handler(d, re).nil();
    let code = effect::get_label(d, a.e(0));
    card::is_code(d, rh, &[code as u32])
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetFirstTarget()
    let tg = duel::first_target(d);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ft1=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // local ft2=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let mut ft2 = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // if c:IsLocation(LOCATION_SZONE) then ft2=ft2-1 end
    if card::is_location(d, c, LOCATION_SZONE) {
        ft2 -= 1;
    }
    // if ft1<1 or ft2<0 then return false end
    if ft1 < 1 || ft2 < 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp,e:GetLabel()):GetFirst()
    let lv = effect::get_label(d, a.e) as u32;
    let sg = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, a.e, tp, lv)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sg);
    // if tc and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)>0 then
    let Some(tc) = tc else { return };
    if duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // if tg and tg:IsRelateToEffect(e) then
        if let Some(tg) = tg {
            if card::is_relate_to_effect(d, tg, a.e) {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Equip(tp,tg,tc,true)
                duel::equip(d, tp, tg, tc, true, false);
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetCode(EFFECT_EQUIP_LIMIT)
                effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // e1:SetValue(s.eqlimit)
                effect::set_value_fn(d, e1, effect_card_fn(eqlimit));
                // e1:SetLabelObject(tc)
                effect::set_label_object(d, e1, Value::Card(tc));
                // tg:RegisterEffect(e1)
                card::register_effect(d, tg, e1, false);
                // Gains ATK equal to half its original ATK
                // local atk=tg:GetTextAttack()/2
                let mut atk = card::get_text_attack(d, tg) as f64 / 2.0;
                // if atk<0 then atk=0 end
                if atk < 0.0 {
                    atk = 0.0;
                }
                // local e2=Effect.CreateEffect(c)
                let e2 = effect::create_effect(d, c);
                // e2:SetType(EFFECT_TYPE_EQUIP)
                effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
                // e2:SetCode(EFFECT_UPDATE_ATTACK)
                effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
                // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
                // e2:SetValue(atk)
                effect::set_value(d, e2, lua_int(atk));
                // tg:RegisterEffect(e2)
                card::register_effect(d, tg, e2, false);
                // Cannot activate cards/effects with the same name
                // local e3=Effect.CreateEffect(c)
                let e3 = effect::create_effect(d, c);
                // e3:SetType(EFFECT_TYPE_FIELD)
                effect::set_type(d, e3, EFFECT_TYPE_FIELD);
                // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
                effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
                // e3:SetCode(EFFECT_CANNOT_ACTIVATE)
                effect::set_code(d, e3, EFFECT_CANNOT_ACTIVATE);
                // e3:SetTargetRange(1,0)
                effect::set_target_range(d, e3, 1, 0);
                // e3:SetValue(s.aclimit)
                effect::set_value_fn(d, e3, lua_fn(aclimit));
                // e3:SetLabel(tg:GetCode())
                let code = card::get_code(d, tg)[0];
                effect::set_label(d, e3, &[code as i64]);
                // e3:SetReset(RESET_PHASE|PHASE_END)
                effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
                // Duel.RegisterEffect(e3,tp)
                duel::register_effect(d, e3, tp);
            }
        }
    }
}
