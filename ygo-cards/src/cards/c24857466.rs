//! Van'Dalgyon the Dark Dragon Lord
//! (冥王竜ヴァンダルギオン)
use super::prelude::*;

pub const ID: u32 = 24857466;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAIN_NEGATED)
    effect::set_code(d, e1, EVENT_CHAIN_NEGATED);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetOperation(s.chop)
    effect::set_operation(d, e1, Some(operation(chop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_CHAIN_END)
    effect::set_code(d, e2, EVENT_CHAIN_END);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCondition(s.hspcon)
    effect::set_condition(d, e2, Some(condition(hspcon)));
    // e2:SetTarget(s.hsptg)
    effect::set_target(d, e2, Some(cost_target(hsptg)));
    // e2:SetOperation(s.hspop)
    effect::set_operation(d, e2, Some(operation(hspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // spell:damage
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e3, EVENT_CUSTOM + ID);
    // e3:SetCondition(s.damcon)
    effect::set_condition(d, e3, Some(condition(damcon)));
    // e3:SetTarget(s.damtg)
    effect::set_target(d, e3, Some(cost_target(damtg)));
    // e3:SetOperation(s.damop)
    effect::set_operation(d, e3, Some(operation(damop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // trap:Destroy
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_DESTROY);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e4, EVENT_CUSTOM + ID);
    // e4:SetCondition(s.descon)
    effect::set_condition(d, e4, Some(condition(descon)));
    // e4:SetTarget(s.destg)
    effect::set_target(d, e4, Some(cost_target(destg)));
    // e4:SetOperation(s.desop)
    effect::set_operation(d, e4, Some(operation(desop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // monster:spsummon
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e5, aux::stringid(ID, 3));
    // e5:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e5, CATEGORY_SPECIAL_SUMMON);
    // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e5:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e5, EFFECT_FLAG_CARD_TARGET, 0);
    // e5:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e5, EVENT_CUSTOM + ID);
    // e5:SetCondition(s.spcon)
    effect::set_condition(d, e5, Some(condition(spcon)));
    // e5:SetTarget(s.sptg)
    effect::set_target(d, e5, Some(cost_target(sptg)));
    // e5:SetOperation(s.spop)
    effect::set_operation(d, e5, Some(operation(spop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.chop(e,tp,eg,ep,ev,re,r,rp)`
fn chop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if rp==tp or (not re:IsMonsterEffect() and not re:IsHasType(EFFECT_TYPE_ACTIVATE)) then return end
    let re = a.re.nil();
    if a.rp == tp || (!effect::is_monster_effect(d, re) && !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)) {
        return;
    }
    // local de,dp=Duel.GetChainInfo(ev,CHAININFO_DISABLE_REASON,CHAININFO_DISABLE_PLAYER)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_DISABLE_REASON, CHAININFO_DISABLE_PLAYER]);
    let de = info[0].as_effect();
    let dp_is_tp = matches!(info[1], Value::Int(x) if x == tp as i64);
    // if de and dp==tp and de:GetHandler():IsType(TYPE_COUNTER) then
    if let Some(de) = de {
        let deh = effect::get_handler(d, de).nil();
        if dp_is_tp && card::is_type(d, deh, TYPE_COUNTER) {
            // local ty=re:GetActiveType()
            let ty = effect::get_active_type(d, re);
            // local flag=c:GetFlagEffectLabel(id)
            let flag = card::get_flag_effect_label(d, c, ID).first().copied();
            match flag {
                // if not flag then
                None => {
                    // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,0,ty)
                    card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD, 0, 0, ty as i64, 0);
                    // e:SetLabelObject(de)
                    effect::set_label_object(d, a.e, Value::Effect(de));
                }
                // elseif de~=e:GetLabelObject() then
                Some(flag) => {
                    if effect::get_label_object(d, a.e).as_effect() != Some(de) {
                        // e:SetLabelObject(de)
                        effect::set_label_object(d, a.e, Value::Effect(de));
                        // c:SetFlagEffectLabel(id,ty)
                        card::set_flag_effect_label(d, c, ID, ty);
                    } else {
                        // c:SetFlagEffectLabel(id,(flag|ty))
                        card::set_flag_effect_label(d, c, ID, (flag as u32) | ty);
                    }
                }
            }
        }
    }
}
/// `s.hspcon(e,tp,eg,ep,ev,re,r,rp)`
fn hspcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local label=c:GetFlagEffectLabel(id)
    let label = card::get_flag_effect_label(d, c, ID).first().copied();
    // if label~=nil and label~=0 then
    match label {
        Some(label) if label != 0 => {
            // e:SetLabel(label)
            effect::set_label(d, a.e, &[label]);
            // c:SetFlagEffectLabel(id,0)
            card::set_flag_effect_label(d, c, ID, 0);
            // return true
            true
        }
        // else return false end
        _ => false,
    }
}
/// `s.hsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn hsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetTargetParam(e:GetLabel())
    duel::set_target_param(d, effect::get_label(d, e) as u32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.hspop(e,tp,eg,ep,ev,re,r,rp)`
fn hspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // if Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)~=0 then
    if duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) != 0 {
        // local tpe=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
        let tpe = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
        // Duel.RaiseSingleEvent(c,EVENT_CUSTOM+id,e,0,0,tp,tpe)
        duel::raise_single_event(d, c, EVENT_CUSTOM + ID, Some(e), 0, 0, tp, tpe as u32);
    }
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return (ev&TYPE_SPELL)~=0
    (a.ev & TYPE_SPELL) != 0
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - tp);
    // Duel.SetTargetParam(1500)
    duel::set_target_param(d, 1500);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,1500)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 1500);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return (ev&TYPE_TRAP)~=0
    (a.ev & TYPE_TRAP) != 0
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsOnField() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_on_field(d, chkc);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.TRUE,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, None, tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return (ev&TYPE_MONSTER)~=0
    (a.ev & TYPE_MONSTER) != 0
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && spfilter(d, chkc, e, tp);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
