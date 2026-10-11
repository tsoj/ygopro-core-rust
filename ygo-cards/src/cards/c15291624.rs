//! Thunder Dragon Colossus
use super::prelude::*;

pub const ID: u32 = 15291624;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Summon procedure
    // Fusion.AddProcMixRep(c,true,true,aux.FilterBoolFunctionEx(Card.IsRace,RACE_THUNDER),1,1,31786629)
    fusion::add_proc_mix_rep(
        d,
        c,
        true,
        true,
        fusion::Mat::F(fusion::fus_mat(|d, c, _fc, _st, _tp| card::is_race(d, c, RACE_THUNDER))),
        1,
        1,
        vec![fusion::Mat::Code(31786629)],
    );
    // Special Summon condition
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(aux.fuslimit)
    effect::set_value_fn(d, e1, aux::fuslimit());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Aternative Special Summon procedure
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e2, LOCATION_EXTRA);
    // e2:SetCondition(s.hspcon)
    effect::set_condition(d, e2, Some(lua_fn(|d, a: Args| hspcon(d, a.e(0), a.card(1)))));
    // e2:SetTarget(s.hsptg)
    effect::set_target(d, e2, Some(lua_fn(hsptg)));
    // e2:SetOperation(s.hspop)
    effect::set_operation(d, e2, Some(lua_fn(hspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Prevents search
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CANNOT_TO_HAND)
    effect::set_code(d, e3, EFFECT_CANNOT_TO_HAND);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(0,LOCATION_DECK)
    effect::set_target_range(d, e3, 0, LOCATION_DECK);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Destruction replacement
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e4:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e4:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e4, EFFECT_DESTROY_REPLACE);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetTarget(s.desreptg)
    effect::set_target(d, e4, Some(cost_target(desreptg)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_CHAIN,s.chainfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_CHAIN, lua_fn(chainfilter));
}
/// `s.chainfilter(re,tp,cid)`
fn chainfilter(d: &mut Duel, a: Args) -> bool {
    // return not (re:IsMonsterEffect() and re:GetHandler():IsRace(RACE_THUNDER)
    //   and (Duel.GetChainInfo(cid,CHAININFO_TRIGGERING_LOCATION)==LOCATION_HAND))
    let re = a.e(0);
    let cid = a.int(2) as u8;
    let mut hit = effect::is_monster_effect(d, re);
    if hit {
        let rc = effect::get_handler(d, re).nil();
        hit = card::is_race(d, rc, RACE_THUNDER);
    }
    if hit {
        hit = duel::get_chain_info(d, cid, &[CHAININFO_TRIGGERING_LOCATION])[0].int() == LOCATION_HAND as i64;
    }
    !hit
}
/// `s.hspfilter(c,tp,sc)`
fn hspfilter(d: &mut Duel, c: CardId, tp: u8, sc: CardId) -> bool {
    // return c:IsRace(RACE_THUNDER) and not c:IsType(TYPE_FUSION,sc,MATERIAL_FUSION,tp)
    //   and c:IsType(TYPE_EFFECT,sc,MATERIAL_FUSION,tp) and Duel.GetLocationCountFromEx(tp,tp,c,sc)>0
    card::is_race(d, c, RACE_THUNDER)
        && !card::is_type_ctx(d, c, TYPE_FUSION, Some(sc), MATERIAL_FUSION as u64, Some(tp))
        && card::is_type_ctx(d, c, TYPE_EFFECT, Some(sc), MATERIAL_FUSION as u64, Some(tp))
        && duel::get_location_count_from_ex(d, tp, Some(tp), c.into(), duel::ExtraCard::Card(sc), None).0 > 0
}
/// `s.hspcon(e,c)`
fn hspcon(d: &mut Duel, _e: EffectId, c: Option<CardId>) -> bool {
    // if c==nil then return true end
    let Some(c) = c else { return true };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.CheckReleaseGroup(c:GetControler(),s.hspfilter,1,false,1,true,c,c:GetControler(),nil,false,nil,tp,c)
    //   and (Duel.GetCustomActivityCount(id,tp,ACTIVITY_CHAIN)~=0 or Duel.GetCustomActivityCount(id,1-tp,ACTIVITY_CHAIN)~=0)
    let ctrl = card::get_controler(d, c);
    let ra = duel::ReleaseArgs {
        use_hand: Some(false),
        max: Some(1),
        check_field: true,
        to_check: Some(c),
        toplayer: Some(ctrl),
        ..Default::default()
    };
    duel::check_release_group(d, ctrl, Some(&|d, x| hspfilter(d, x, tp, c)), 1, ra, NO_EX, false)
        && (duel::get_custom_activity_count(d, ID, tp, ACTIVITY_CHAIN) != 0 || duel::get_custom_activity_count(d, ID, 1 - tp, ACTIVITY_CHAIN) != 0)
}
/// `s.hsptg(e,tp,eg,ep,ev,re,r,rp,chk,c)`
fn hsptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.c(9);
    // local g=Duel.SelectReleaseGroup(tp,s.hspfilter,1,1,false,true,true,c,nil,nil,false,nil,tp,c)
    let ra = duel::ReleaseArgs {
        use_hand: Some(false),
        cancelable: true,
        check_field: true,
        to_check: Some(c),
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
fn hspop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let c = a.c(8);
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, e).as_group().cloned().nil();
    // Duel.Release(g,REASON_COST|REASON_MATERIAL)
    duel::release(d, g.clone(), REASON_COST | REASON_MATERIAL, None);
    // c:SetMaterial(g)
    card::set_material(d, c, g);
    // g:DeleteGroup()
}
/// `s.repfilter(c)`
fn repfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_THUNDER) and c:IsAbleToRemove() and aux.SpElimFilter(c,true)
    card::is_race(d, c, RACE_THUNDER) && card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.desreptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then
    if a.chk == Some(0) {
        // return not c:IsReason(REASON_REPLACE) and Duel.IsExistingMatchingCard(s.repfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,c)
        return !card::is_reason(d, c, REASON_REPLACE) && duel::is_existing_matching_card(d, Some(&repfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, c.into());
    }
    // if Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESREPLACE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESREPLACE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.repfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,c)
        let g = duel::select_matching_card(d, tp, Some(&repfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, c.into()).nil();
        // Duel.Remove(g:GetFirst(),POS_FACEUP,REASON_EFFECT)
        if let Some(tc) = group::get_first(d, &g) {
            duel::remove(d, CardSet::from_card(tc), POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
        // return true
        true
    } else {
        // else return false end
        false
    }
}
