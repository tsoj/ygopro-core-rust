//! Vaylantz Genesis Grand Duke
use super::prelude::*;

pub const ID: u32 = 76075139;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_VAYLANTZ],
    ..ScriptInfo::DEFAULT
};

/// `s.mask` (local): the Main Monster Zones the Grand Duke can be Special Summoned to from the Extra Deck
const MASK: u32 = 0x2 | 0x8 | 0x20 | 0x40;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c,false)
    pendulum::add_procedure(d, c, Some(false), None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // 2 "Valiants" monsters
    // Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_VAYLANTZ),2)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_VAYLANTZ as u16], Some(fc), st, Some(tp)))), 2)],
    );
    // Special Summon limitation
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    // e0:SetValue(s.splimit)
    effect::set_value_fn(d, e0, lua_fn(splimit));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Alternate summon procedure
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e1, LOCATION_EXTRA);
    // e1:SetCondition(s.hspcon)
    effect::set_condition(d, e1, Some(lua_fn(hspcon)));
    // e1:SetTarget(s.hsptg)
    effect::set_target(d, e1, Some(lua_fn(hsptg)));
    // e1:SetOperation(s.hspop)
    effect::set_operation(d, e1, Some(operation(hspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon self or move 1 "Valiants" monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.spmvtg)
    effect::set_target(d, e2, Some(cost_target(spmvtg)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Send Monster Card to hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_DAMAGE+CATEGORY_ATKCHANGE)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_DAMAGE + CATEGORY_ATKCHANGE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.splimit(e,se,sp,st)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // return not e:GetHandler():IsLocation(LOCATION_EXTRA) or aux.fuslimit(e,se,sp,st) or aux.penlimit(e,se,sp,st)
    let h = effect::get_handler(d, e).nil();
    !card::is_location(d, h, LOCATION_EXTRA)
        || aux::fuslimit().call(d, &a.0[..4]).truthy()
        || aux::penlimit().call(d, &a.0[..4]).truthy()
}
/// `s.hspfilter(c,tp,sc)`
fn hspfilter(d: &mut Duel, c: CardId, tp: u8, sc: CardId) -> bool {
    // local zone=1<<c:GetSequence()
    let zone = 1u32 << card::get_sequence(d, c);
    // return zone&mask==zone and c:IsSetCard(SET_VAYLANTZ) and c:IsLevelAbove(5)
    //   and not c:IsType(TYPE_FUSION) and Duel.GetLocationCountFromEx(tp,tp,c,sc)>0
    (zone & MASK) == zone
        && card::is_set_card(d, c, &[SET_VAYLANTZ as u16])
        && card::is_level_above(d, c, 5)
        && !card::is_type(d, c, TYPE_FUSION)
        && duel::get_location_count_from_ex(d, tp, Some(tp), c.into(), duel::ExtraCard::Card(sc), None).0 > 0
}
/// `s.hspcon(e,c)`
fn hspcon(d: &mut Duel, a: Args) -> bool {
    // if not c then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // if c:IsFaceup() then return false end
    if card::is_faceup(d, c) {
        return false;
    }
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.CheckReleaseGroup(tp,s.hspfilter,1,false,1,true,c,tp,nil,nil,nil,tp,c)
    let ra = duel::ReleaseArgs {
        use_hand: Some(false),
        max: Some(1),
        check_field: true,
        to_check: Some(c),
        toplayer: Some(tp),
        ..Default::default()
    };
    duel::check_release_group(d, tp, Some(&|d, x| hspfilter(d, x, tp, c)), 1, ra, NO_EX, false)
}
/// `s.hsptg(e,tp,eg,ep,ev,re,r,rp,chk,c)`
fn hsptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.c(9);
    // local g=Duel.SelectReleaseGroup(tp,s.hspfilter,1,1,false,true,true,c,tp,nil,false,nil,tp,c)
    let ra = duel::ReleaseArgs {
        use_hand: Some(false),
        cancelable: true,
        check_field: true,
        to_check: Some(c),
        toplayer: Some(tp),
        ..Default::default()
    };
    let g = duel::select_release_group(d, tp, Some(&|d, x| hspfilter(d, x, tp, c)), 1, 1, ra, NO_EX, false);
    // if g then
    if let Some(g) = g {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        true
    } else {
        // return false
        false
    }
}
/// `s.hspop(e,tp,eg,ep,ev,re,r,rp,c)`
fn hspop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned();
    // if not g then return end
    let Some(g) = g else {
        return;
    };
    // Duel.Release(g,REASON_COST|REASON_MATERIAL)
    duel::release(d, g, REASON_COST | REASON_MATERIAL, None);
    // g:DeleteGroup()
}
/// `s.spmvtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spmvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local sp=s.sptg(e,tp,eg,ep,ev,re,r,rp,0)
    let mut a0 = a.clone();
    a0.chk = Some(0);
    let sp = sptg(d, &a0);
    // local mv=s.mvtg(e,tp,eg,ep,ev,re,r,rp,0)
    let mv = mvtg(d, &a0);
    // if chk==0 then return sp or mv end
    if a.chk == Some(0) {
        return sp || mv;
    }
    // local op=Duel.SelectEffect(tp,{sp,aux.Stringid(id,2)},{mv,aux.Stringid(id,3)})
    let op = duel::select_effect(d, a.tp, &[(sp, aux::stringid(ID, 2)), (mv, aux::stringid(ID, 3))]);
    // if op==1 then
    if op == Some(1) {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // e:SetOperation(s.spop)
        effect::set_operation(d, a.e, Some(operation(spop)));
        // s.sptg(e,tp,eg,ep,ev,re,r,rp,1)
        let mut a1 = a.clone();
        a1.chk = Some(1);
        sptg(d, &a1);
    } else if op == Some(2) {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
        // e:SetOperation(s.mvop)
        effect::set_operation(d, a.e, Some(operation(mvop)));
    }
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then
    if a.chk == Some(0) {
        // local zone=(1<<c:GetSequence())&ZONES_MMZ
        let zone = (1u32 << card::get_sequence(d, c)) & ZONES_MMZ;
        // return zone~=0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp,zone)
        return zone != 0 && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, Some(POS_FACEUP as u8), Some(a.tp), Some(zone));
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(c.into()), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local zone=(1<<c:GetSequence())&ZONES_MMZ
    let zone = (1u32 << card::get_sequence(d, c)) & ZONES_MMZ;
    // if zone~=0 then
    if zone != 0 {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP,zone)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, Some(zone));
    }
}
/// `s.mvtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn mvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingTarget(Card.CheckAdjacent,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::check_adjacent(d, c)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // (no return value in Lua: nil)
    false
}
/// `s.mvop(e,tp,eg,ep,ev,re,r,rp)`
fn mvop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SELECT)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SELECT as u64);
    // local tc=Duel.SelectMatchingCard(tp,Card.CheckAdjacent,tp,LOCATION_MZONE,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&|d, c| card::check_adjacent(d, c)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if tc then
    if let Some(tc) = group::get_first(d, &sel) {
        // tc:MoveAdjacent()
        card::move_adjacent(d, tc, None);
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsOriginalType(TYPE_MONSTER) and c:GetSequence()<5 and c:GetBaseAttack()>0 and c:IsAbleToHand()
    card::is_original_type(d, c, TYPE_MONSTER) && card::get_sequence(d, c) < 5 && card::get_base_attack(d, c) > 0 && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_SZONE) and chkc:IsControler(1-tp) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_SZONE) && card::is_controler(d, chkc, 1 - a.tp) && thfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,0,LOCATION_SZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), a.tp, 0, LOCATION_SZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,0,LOCATION_SZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, 0, LOCATION_SZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,0,0)
    let n = g.clone().nil().len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g.clone(), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,g:GetFirst():GetBaseAttack())
    let first = group::get_first(d, &g.clone().nil()).nil();
    let atk = card::get_base_attack(d, first);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, atk);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and Duel.SendtoHand(tc,nil,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_HAND)) then return end
    if !(card::is_relate_to_effect(d, tc, a.e)
        && duel::send_to_hand(d, tc, None, REASON_EFFECT, None) > 0
        && card::is_location(d, tc, LOCATION_HAND))
    {
        return;
    }
    // local atk=tc:GetBaseAttack()
    let atk = card::get_base_attack(d, tc);
    // if atk<=0 then return end
    if atk <= 0 {
        return;
    }
    // local dam=Duel.Damage(1-tp,atk,REASON_EFFECT)
    let dam = duel::damage(d, 1 - a.tp, atk as i64, REASON_EFFECT, false, None);
    // if dam>0 then
    if dam > 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(dam/2)
        effect::set_value(d, e1, lua_int(dam as f64 / 2.0));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
