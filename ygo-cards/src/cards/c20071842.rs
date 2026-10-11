//! Heavy Interlock
//! Scripted by Eerie Code
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 20071842;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_names={7987191}
    listed_names: &[7987191],
    // s.fit_monster={7987191} --should be removed in hardcode overhaul
    fit_monster: &[7987191],
    // s.listed_series={SET_ROKKET}
    listed_series: &[SET_ROKKET],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual.AddProcGreater{handler=c,filter=s.ritualfil,lv=8,extrafil=s.extrafil,extraop=s.extraop,stage2=s.stage2}
    ritual::add_proc_greater(
        d,
        c,
        ritual::RitualProc {
            filter: Some(Arc::new(|d: &mut Duel, x: CardId| ritualfil(d, x))),
            lv: Some(ritual::Lv::N(8)),
            extrafil: Some(Arc::new(|d: &mut Duel, a: &EffArgs| extrafil(d, a))),
            extraop: Some(Arc::new(|d: &mut Duel, mg: &CardSet, a: &EffArgs, tc: CardId| extraop(d, mg, a, tc))),
            stage2: Some(Arc::new(|d: &mut Duel, mat: &CardSet, a: &EffArgs, tc: CardId| stage2(d, mat, a, tc))),
            ..Default::default()
        },
    );
}
// function s.ritualfil(c)
fn ritualfil(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(7987191) and c:IsRitualMonster()
    card::is_code(d, c, &[7987191]) && card::is_ritual_monster(d, c)
}
// function s.mfilter(c,e)
fn mfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return (c:IsLocation(LOCATION_HAND) or c:IsFaceup()) and c:HasLevel() and c:IsSetCard(SET_ROKKET) and c:IsMonster() and c:IsDestructable(e)
    (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
        && card::has_level(d, c)
        && card::is_set_card(d, c, &[SET_ROKKET as u16])
        && card::is_monster(d, c)
        && card::is_destructable(d, c, Some(e))
}
// function s.extrafil(e,tp,eg,ep,ev,re,r,rp,chk)
fn extrafil(d: &mut Duel, a: &EffArgs) -> Option<CardSet> {
    // return Duel.GetMatchingGroup(s.mfilter,tp,LOCATION_MZONE|LOCATION_HAND,0,nil,e)
    let e = a.e;
    Some(duel::get_matching_group(d, Some(&|d, c| mfilter(d, c, e)), a.tp, LOCATION_MZONE | LOCATION_HAND, 0, NO_EX))
}
// function s.extraop(mg,e,tp,eg,ep,ev,re,r,rp)
fn extraop(d: &mut Duel, mg: &CardSet, a: &EffArgs, _tc: CardId) {
    let tp = a.tp;
    // (mg is a Lua group reference: a local copy here, the changes stay within this function)
    let mut mg = mg.clone();
    // local rg=mg:Filter(s.mfilter,nil,e)
    let e = a.e;
    let rg = group::filter(d, &mg, &|d, c| mfilter(d, c, e), NO_EX);
    // local mat2=Group.CreateGroup()
    let mut mat2 = CardSet::new();
    // if #rg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if !rg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // mat2=rg:FilterSelect(tp,s.mfilter,1,#rg,nil,e)
        let n = rg.len() as u16;
        mat2 = group::filter_select(d, &rg, tp, &|d, c| mfilter(d, c, e), 1, n, false, NO_EX).nil();
        // mg:Sub(mat2)
        group::remove_card(&mut mg, &mat2);
    }
    // Duel.ReleaseRitualMaterial(mg)
    duel::release_ritual_material(d, &mg, false);
    // Duel.Destroy(mat2,REASON_EFFECT)
    duel::destroy(d, mat2, REASON_EFFECT, None, None);
}
// function s.stage2(mat,e,tp,eg,ep,ev,re,r,rp,tc)
fn stage2(d: &mut Duel, _mat: &CardSet, a: &EffArgs, tc: CardId) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    // e1:SetValue(s.efilter)
    effect::set_value_fn(d, e1, lua_fn(efilter));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // tc:RegisterEffect(e1,true)
    card::register_effect(d, tc, e1, true);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetValue(s.indval)
    effect::set_value_fn(d, e2, effect_card_fn(|d, _e, c| indval(d, c)));
    // tc:RegisterEffect(e2,true)
    card::register_effect(d, tc, e2, true);
}
// function s.efilter(e,te)
fn efilter(d: &mut Duel, a: Args) -> bool {
    let te = a.e(1);
    // local tc=te:GetOwner()
    let tc = effect::get_owner(d, te).nil();
    // return tc:IsSpecialSummoned() and tc:IsSummonLocation(LOCATION_EXTRA) and te:IsMonsterEffect()
    //     and te:IsActivated() and te:GetActivateLocation()==LOCATION_MZONE
    card::is_special_summoned(d, tc)
        && card::is_summon_location(d, tc, LOCATION_EXTRA)
        && effect::is_monster_effect(d, te)
        && effect::is_activated(d, te)
        && effect::get_activate_location(d, te) == LOCATION_MZONE
}
// function s.indval(e,c)
fn indval(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSummonLocation(LOCATION_EXTRA)
    card::is_summon_location(d, c, LOCATION_EXTRA)
}
